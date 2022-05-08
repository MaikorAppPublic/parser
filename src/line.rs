use crate::{EmptyLine, get_op_code, ParserError, to_args_str, Token};
use crate::tokens::create_token;

pub fn parse_line(line: &str) -> Result<(u8, Vec<u8>), ParserError> {
    if line.is_empty() {
        return Err(EmptyLine);
    }
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err(EmptyLine);
    }
    //find a better solution for this
    let promote_bytes = parts[0].to_ascii_uppercase().ends_with('W');
    let args = if parts.len() > 1 {
        parse_args(&parts[1..], promote_bytes)?
    } else {
        vec![]
    };

    let bytes = args.iter().flat_map(Vec::<u8>::from).collect();
    let pattern = &to_args_str(&args);
    let op = get_op_code(parts[0], pattern)?;
    Ok((op, bytes))
}

fn parse_args(args: &[&str], promote_bytes: bool) -> Result<Vec<Token>, ParserError> {
    let mut list = vec![];
    for &arg in args {
        list.push(create_token(arg, promote_bytes)?);
    }
    Ok(list)
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_language::op_desc;
    use maikor_language::op_params::{INDIRECT, IND_POST_INC, PRE_DEC, IND_POST_DEC, POST_INC, PRE_INC, IND_PRE_DEC};
    use maikor_language::ops::*;
    use maikor_language::registers::offset;
    use offset::*;

    fn assert_op(line: &str, expected_op: u8, expected_args: Vec<u8>) {
        let (op, args) = parse_line(line).unwrap();
        assert_eq!(
            expected_op,
            op,
            "{line} (expected: '{}', parsed: '{}')",
            op_desc(expected_op).unwrap(),
            op_desc(op).unwrap_or("?")
        );
        assert_eq!(args, expected_args, "{line} args mismatch");
    }

    #[test]
    fn test_basic_parsing() {
        assert_op("INC.B   AL", INC_REG_BYTE, vec![AL as u8]);
        assert_op("INC.W AX", INC_REG_WORD, vec![AX as u8]);
        assert_op(
            "INC.B (BX)",
            INC_REG_BYTE,
            vec![BX as u8 | INDIRECT],
        );
        assert_op(
            "INC.W (AX)",
            INC_REG_WORD,
            vec![AX as u8 | INDIRECT],
        );
        assert_op("INC.B $100", INC_ADDR_BYTE, vec![0, 100]);
        assert_op("INC.W $x101", INC_ADDR_WORD, vec![1, 1]);
        assert_op("INC.W -BX", INC_REG_WORD, vec![BX as u8 | PRE_DEC]);
        assert_op(
            "INC.B (AX)+",
            INC_REG_BYTE,
            vec![AX as u8 | IND_POST_INC],
        );

        assert_op("DEC.B FLG", DEC_REG_BYTE, vec![FLAGS as u8]);
        assert_op(
            "DEC.W (DX)",
            DEC_REG_WORD,
            vec![DX as u8 | INDIRECT],
        );

        assert_op("ADD.B CL 1", ADD_REG_NUM_BYTE, vec![CL as u8, 1]);
    }

    #[test]
    fn test_every_other() {
        assert_op("NOP", NOP, vec![]);
        assert_op("HALT", HALT, vec![]);
        assert_op("RET", RET, vec![]);
        assert_op("RETI", RETI, vec![]);
        assert_op("JRF 10", JRF_BYTE, vec![10]);
        assert_op("JRB x9", JRB_BYTE, vec![9]);
        assert_op("SWAP.B AL AH", SWAP_REG_REG_BYTE, vec![AL as u8, AH as u8]);
        assert_op("SWAP.B (BX) AH", SWAP_REG_REG_BYTE, vec![BX as u8 | INDIRECT, AH as u8]);
        assert_op("SWAP.B AL (BX)", SWAP_REG_REG_BYTE, vec![AL as u8, BX as u8 | INDIRECT]);
        assert_op("SWAP.B (BX) (DX)", SWAP_REG_REG_BYTE, vec![BX as u8 | INDIRECT, DX as u8 | INDIRECT]);
        assert_op("SWAP.W AX CX", SWAP_REG_REG_WORD, vec![AX as u8, CX as u8]);
        assert_op("SWAP.W (AX) CX", SWAP_REG_REG_WORD, vec![AX as u8 | INDIRECT, CX as u8]);
        assert_op("SWAP.W AX (CX)", SWAP_REG_REG_WORD, vec![AX as u8, CX as u8 | INDIRECT]);
        assert_op("SWAP.W (AX) (CX)", SWAP_REG_REG_WORD, vec![AX as u8 | INDIRECT, CX as u8 | INDIRECT]);
    }

    #[test]
    fn test_every_jump() {
        // this test relies on op code ordering matching for JE, JNE, etc
        for (name,code) in [("CALL", CALL_ADDR),("JMP", JMP_ADDR), ("JL", JL_ADDR), ("JE", JE_ADDR), ("JNE", JNE_ADDR), ("JG", JG_ADDR), ("JGE", JGE_ADDR)] {
            // A, E, I
            assert_op(&format!("{name} $10"), code, vec![0,10]);
            assert_op(&format!("{name} AX"), code+1, vec![AX as u8]);
            assert_op(&format!("{name} (AX)"), code+1, vec![AX as u8 | INDIRECT]);
        }

        for (name, code) in [("JBC", JBC_REG_REG), ("JBS", JBS_REG_REG)] {
            // ER, IR, AR, EB, IB, AB
            assert_op(&format!("{name} AX BL"), code, vec![AX as u8, BL as u8]);
            assert_op(&format!("{name} (AX) BL"), code, vec![AX as u8 | INDIRECT, BL as u8]);
            assert_op(&format!("{name} $45 BL"), code+2, vec![0, 45, BL as u8]);
            assert_op(&format!("{name} CX 56"), code+4, vec![CX as u8, 56]);
            assert_op(&format!("{name} (CX) 56"), code+4, vec![CX as u8 | INDIRECT, 56]);
            assert_op(&format!("{name} $45 1"), code+6, vec![0, 45, 1]);
        }
    }

    #[test]
    fn test_every_incdec() {
        //INC.B (R, I, A)
        assert_op("INC.B AL", INC_REG_BYTE, vec![AL as u8]);
        assert_op("INC.B (DX)", INC_REG_BYTE, vec![DX as u8 | INDIRECT]);
        assert_op("INC.B $100", INC_ADDR_BYTE, vec![0, 100]);
        //DEC.B (R, I, A)
        assert_op("DEC.B CH+", DEC_REG_BYTE, vec![CH as u8 | POST_INC]);
        assert_op("DEC.B (BX)-", DEC_REG_BYTE, vec![BX as u8 | IND_POST_DEC]);
        assert_op("DEC.B $xF12", DEC_ADDR_BYTE, vec![15, 18]);
        //INC.W (E, I, A)
        assert_op("INC.W DX", INC_REG_WORD, vec![DX as u8]);
        assert_op("INC.W (AX)", INC_REG_WORD, vec![AX as u8 | INDIRECT]);
        assert_op("INC.W $13", INC_ADDR_WORD, vec![0, 13]);
        //DEC.W (E, I, A)
        assert_op("DEC.W +BX", DEC_REG_WORD, vec![BX as u8 | PRE_INC]);
        assert_op("DEC.W (AX)-", DEC_REG_WORD, vec![AX as u8 | IND_POST_DEC]);
        assert_op("DEC.W $xFFF", DEC_ADDR_WORD, vec![15, 255]);
    }

    #[test]
    fn test_every_logic() {
        // this test relies on op code ordering matching for AND, OR, etc
        check_logic("AND", AND_REG_REG_BYTE, AND_REG_REG_WORD);
        check_logic("OR", OR_REG_REG_BYTE, OR_REG_REG_WORD);
        check_logic("XOR",XOR_REG_REG_BYTE, XOR_REG_REG_WORD);
        assert_op("NOT.B AL",NOT_REG_BYTE, vec![AL as u8]);
        assert_op("NOT.B (BX)",NOT_REG_BYTE, vec![BX as u8 | INDIRECT]);
        assert_op("NOT.W CX",NOT_REG_WORD, vec![CX as u8]);
        assert_op("NOT.W (DX)",NOT_REG_WORD, vec![DX as u8 | INDIRECT]);
    }

    fn check_logic(command :&str, byte_start: u8, word_start: u8) {
        //RR, RI, IR, II, IB, RB
        assert_op(&format!("{command}.B AL AH"), byte_start, vec![AL as u8, AH as u8]);
        assert_op(&format!("{command}.B AL (BX)"), byte_start, vec![AL as u8, BX as u8 | INDIRECT]);
        assert_op(&format!("{command}.B (BX) AH"), byte_start, vec![BX as u8 | INDIRECT, AH as u8]);
        assert_op(&format!("{command}.B (CX) (DX)"), byte_start, vec![CX as u8 | INDIRECT, DX as u8 | INDIRECT]);
        assert_op(&format!("{command}.B (CX) 10"), byte_start + 2, vec![CX as u8 | INDIRECT, 10]);
        assert_op(&format!("{command}.B BH 25"), byte_start + 2, vec![BH as u8, 25]);

        //EE, EI, IE, II, IW, RW
        assert_op(&format!("{command}.W AX BX"), word_start, vec![AX as u8, BX as u8]);
        assert_op(&format!("{command}.W CX (BX)"), word_start, vec![CX as u8, BX as u8 | INDIRECT]);
        assert_op(&format!("{command}.W (BX) DX"), word_start, vec![BX as u8 | INDIRECT, DX as u8]);
        assert_op(&format!("{command}.W (AX) (DX)"), word_start, vec![AX as u8 | INDIRECT, DX as u8 | INDIRECT]);
        assert_op(&format!("{command}.W (CX) 10"), word_start + 2, vec![CX as u8 | INDIRECT, 0, 10]);
        assert_op(&format!("{command}.W DX 2567"), word_start + 2, vec![DX as u8, 10, 7]);
    }

    #[test]
    fn test_every_math() {
        // this test relies on op code ordering matching for ADD, SUB, etc
        check_math_ops("ADD", ADD_REG_REG_BYTE, ADD_REG_REG_WORD);
        check_math_ops("SUB", SUB_REG_REG_BYTE, SUB_REG_REG_WORD);
        check_math_ops("MUL", MUL_REG_REG_BYTE, MUL_REG_REG_WORD);
        check_math_ops("DIV", DIV_REG_REG_BYTE, DIV_REG_REG_WORD);
        check_math_ops("MULS", MULS_REG_REG_BYTE, MULS_REG_REG_WORD);
        check_math_ops("DIVS", DIVS_REG_REG_BYTE, DIVS_REG_REG_WORD);
    }

    fn check_math_ops(command: &str, byte_start: u8, word_start: u8) {
        // IR, RR, RI, II, IB, RB, IA, RA, AR, AI, AB, AA
        assert_op(&format!("{command}.B (AX) CL"), byte_start, vec![AX as u8 | INDIRECT, CL as u8]);
        assert_op(&format!("{command}.B BH CH"), byte_start, vec![BH as u8, CH as u8]);
        assert_op(&format!("{command}.B BH (DX)"), byte_start, vec![BH as u8, DX as u8 | INDIRECT]);
        assert_op(&format!("{command}.B (BX) (DX)+"), byte_start, vec![BX as u8 | INDIRECT, DX as u8 | IND_POST_INC]);
        assert_op(&format!("{command}.B (CX) 25"), byte_start + 2, vec![CX as u8 | INDIRECT, 25]);
        assert_op(&format!("{command}.B -AL 60"), byte_start + 2, vec![AL as u8 | PRE_DEC, 60]);
        assert_op(&format!("{command}.B (AX) $10"), byte_start + 4, vec![AX as u8|INDIRECT, 0, 10]);
        assert_op(&format!("{command}.B CH $x40"), byte_start + 4, vec![CH as u8, 0, 64]);
        assert_op(&format!("{command}.B $256 DL"), byte_start + 6, vec![1,0,DL as u8]);
        assert_op(&format!("{command}.B $x1010 (BX)"), byte_start + 6, vec![16,16,BX as u8 | INDIRECT]);
        assert_op(&format!("{command}.B $65535 10"), byte_start + 8, vec![255,255,10]);
        assert_op(&format!("{command}.B $56 $x1"), byte_start + 10, vec![0,56,0,1]);

        // IE, EE, EI, II, IW, EW, IA, EA, AE, AI, AW, AA
        assert_op(&format!("{command}.W (AX) CX"), word_start, vec![AX as u8 | INDIRECT, CX as u8]);
        assert_op(&format!("{command}.W BX -DX"), word_start, vec![BX as u8, DX as u8 | PRE_DEC]);
        assert_op(&format!("{command}.W AX (CX)+"), word_start, vec![AX as u8, CX as u8 | IND_POST_INC]);
        assert_op(&format!("{command}.W (BX) (DX)"), word_start, vec![BX as u8 | INDIRECT, DX as u8 |INDIRECT]);
        assert_op(&format!("{command}.W (AX) 1200"), word_start + 2, vec![AX as u8 | INDIRECT, 4, 176]);
        assert_op(&format!("{command}.W DX 89"), word_start + 2, vec![DX as u8, 0, 89]);
        assert_op(&format!("{command}.W (BX) $10"), word_start+4, vec![BX as u8|INDIRECT, 0, 10]);
        assert_op(&format!("{command}.W BX $10"), word_start+4, vec![BX as u8, 0, 10]);
        assert_op(&format!("{command}.W $800 BX"), word_start+6, vec![3,32,BX as u8]);
        assert_op(&format!("{command}.W $799 (AX)"), word_start+6, vec![3,31,AX as u8 | INDIRECT]);
        assert_op(&format!("{command}.W $51 567"), word_start+8, vec![0,51,2,55]);
        assert_op(&format!("{command}.W $100 $102"), word_start+10, vec![0,100,0,102]);
    }

    #[test]
    fn test_every_mcpy() {
        //MCPY (AAB, AAR, AAI, AEB, AIB, IAB, EAB, IIB, EIB, IEB, AII, III, EII, IEI, EEI, AIR, AER, IAR, EAR, IAI, EAI, IIR, EIR, EER, IER)
        assert_op("MCPY $1 $1 1", MEM_CPY_ADDR_ADDR_BYTE, vec![0, 1, 0, 1, 1]);
        assert_op("MCPY $xF $2000 AL", MEM_CPY_ADDR_ADDR_REG, vec![0, 15, 7, 208, AL as u8]);
        assert_op("MCPY $11 $x561 (DX)", MEM_CPY_ADDR_ADDR_REG, vec![0, 11, 5, 97, DX as u8 | INDIRECT]);
        assert_op("MCPY $3532 AX 30", MEM_CPY_ADDR_REG_BYTE, vec![13, 204, AX as u8, 30]);
        assert_op("MCPY $3532 (AX) 30", MEM_CPY_ADDR_REG_BYTE, vec![13, 204, AX as u8 | INDIRECT, 30]);
        assert_op("MCPY (DX)+ $1 255", MEM_CPY_REG_ADDR_BYTE, vec![DX as u8 | IND_POST_INC, 0, 1, 255]);
        assert_op("MCPY (AX) (CX) 63", MEM_CPY_REG_REG_BYTE, vec![AX as u8 | INDIRECT, CX as u8 | INDIRECT, 63]);
        assert_op("MCPY BX (CX) 63", MEM_CPY_REG_REG_BYTE, vec![BX as u8, CX as u8 | INDIRECT, 63]);
        assert_op("MCPY (BX) CX 63", MEM_CPY_REG_REG_BYTE, vec![BX as u8 | INDIRECT, CX as u8, 63]);
        assert_op("MCPY $xa (AX) (DX)", MEM_CPY_ADDR_REG_REG, vec![0, 10, AX as u8 | INDIRECT, DX as u8 | INDIRECT]);
        assert_op("MCPY (AX) (BX) (CX)", MEM_CPY_REG_REG_REG, vec![AX as u8 | INDIRECT, BX as u8 | INDIRECT, CX as u8 | INDIRECT]);
        assert_op("MCPY DX (BX) (CX)", MEM_CPY_REG_REG_REG, vec![DX as u8, BX as u8 | INDIRECT, CX as u8 | INDIRECT]);
        assert_op("MCPY (DX) BX (CX)", MEM_CPY_REG_REG_REG, vec![DX as u8 | INDIRECT, BX as u8, CX as u8 | INDIRECT]);
        assert_op("MCPY DX BX (CX)", MEM_CPY_REG_REG_REG, vec![DX as u8, BX as u8, CX as u8 | INDIRECT]);
        assert_op("MCPY $50 (AX) -CL", MEM_CPY_ADDR_REG_REG, vec![0, 50, AX as u8 | INDIRECT, CL as u8 | PRE_DEC]);
        assert_op("MCPY $60 BX AH+", MEM_CPY_ADDR_REG_REG, vec![0, 60, BX as u8, AH as u8 | POST_INC]);
        assert_op("MCPY (AX) $1000 BH", MEM_CPY_REG_ADDR_REG, vec![AX as u8 | INDIRECT, 3, 232, BH as u8]);
        assert_op("MCPY BX $x1010 CH", MEM_CPY_REG_ADDR_REG, vec![BX as u8, 16, 16, CH as u8]);
        assert_op("MCPY -(AX) $10 (BX)+", MEM_CPY_REG_ADDR_REG, vec![AX as u8 | IND_PRE_DEC, 0, 10, BX as u8 | IND_POST_INC]);
        assert_op("MCPY CX $x90 -(DX)", MEM_CPY_REG_ADDR_REG, vec![CX as u8, 0, 144, DX as u8 | IND_PRE_DEC]);
        assert_op("MCPY (AX) (BX) CL", MEM_CPY_REG_REG_REG, vec![AX as u8 | INDIRECT, BX as u8 | INDIRECT, CL as u8]);
        assert_op("MCPY CX (DX) FLG", MEM_CPY_REG_REG_REG, vec![CX as u8, DX as u8 | INDIRECT, FLAGS as u8]);
        assert_op("MCPY AX BX CH", MEM_CPY_REG_REG_REG, vec![AX as u8, BX as u8, CH as u8]);
        assert_op("MCPY (DX) CX AH", MEM_CPY_REG_REG_REG, vec![DX as u8 | INDIRECT, CX as u8, AH as u8]);
    }

    #[test]
    fn basic_invalid_tests() {
        //Doesn't exist
        assert!(parse_line("sadsfhsg").is_err());
        //Missing size
        assert!(parse_line("INC AL").is_err());
        //INC.B (E)
        assert!(parse_line("INC.B AX").is_err());
        //INC.W (R)
        assert!(parse_line("INC.W AL").is_err());
        //DEC.B (E)
        assert!(parse_line("DEC.B BX").is_err());
        //DEC.W (R)
        assert!(parse_line("DEC.W BL").is_err());
    }
}
