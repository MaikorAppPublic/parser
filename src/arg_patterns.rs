use lazy_static::lazy_static;
use maikor_platform::ops::*;
use std::collections::HashMap;

lazy_static! {
    //format is Map<op_name, Map<pattern, op_code>>
    pub static ref ARG_MATCHES: HashMap<&'static str, HashMap<&'static str, u8>> = make_map();
}

fn make_map() -> HashMap<&'static str, HashMap<&'static str, u8>> {
    let mut map = HashMap::new();
    map.insert("NOP", no_args(NOP));
    map.insert("HALT", no_args(HALT));
    map.insert("EHALT", no_args(EHALT));
    map.insert("SLEEP", no_args(SLEEP));
    map.insert(
        "CPY.B",
        make_math_args_map_b(
            CPY_REG_REG_BYTE,
            CPY_ADDR_REG_BYTE,
            CPY_REG_NUM_BYTE,
            CPY_ADDR_NUM_BYTE,
            CPY_REG_ADDR_BYTE,
            CPY_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "CPY.W",
        make_math_args_map_w(
            CPY_REG_REG_WORD,
            CPY_ADDR_REG_WORD,
            CPY_REG_NUM_WORD,
            CPY_ADDR_NUM_WORD,
            CPY_REG_ADDR_WORD,
            CPY_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "CMP.B",
        make_cmp_args_map_b(CMP_REG_REG_BYTE, CMP_REG_NUM_BYTE, CMP_REG_ADDR_BYTE),
    );
    map.insert(
        "CMP.W",
        make_cmp_args_map_w(CMP_REG_REG_WORD, CMP_REG_NUM_WORD, CMP_REG_ADDR_WORD),
    );
    map.insert(
        "CMPS.B",
        make_cmp_args_map_b(CMPS_REG_REG_BYTE, CMPS_REG_NUM_BYTE, CMPS_REG_ADDR_BYTE),
    );
    map.insert(
        "CMPS.W",
        make_cmp_args_map_w(CMPS_REG_REG_WORD, CMPS_REG_NUM_WORD, CMPS_REG_ADDR_WORD),
    );
    map.insert(
        "ADD.B",
        make_math_args_map_b(
            ADD_REG_REG_BYTE,
            ADD_ADDR_REG_BYTE,
            ADD_REG_NUM_BYTE,
            ADD_ADDR_NUM_BYTE,
            ADD_REG_ADDR_BYTE,
            ADD_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "ADD.W",
        make_math_args_map_w(
            ADD_REG_REG_WORD,
            ADD_ADDR_REG_WORD,
            ADD_REG_NUM_WORD,
            ADD_ADDR_NUM_WORD,
            ADD_REG_ADDR_WORD,
            ADD_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "SUB.B",
        make_math_args_map_b(
            SUB_REG_REG_BYTE,
            SUB_ADDR_REG_BYTE,
            SUB_REG_NUM_BYTE,
            SUB_ADDR_NUM_BYTE,
            SUB_REG_ADDR_BYTE,
            SUB_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "SUB.W",
        make_math_args_map_w(
            SUB_REG_REG_WORD,
            SUB_ADDR_REG_WORD,
            SUB_REG_NUM_WORD,
            SUB_ADDR_NUM_WORD,
            SUB_REG_ADDR_WORD,
            SUB_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "ADDC.B",
        make_math_args_map_b(
            ADDC_REG_REG_BYTE,
            ADDC_ADDR_REG_BYTE,
            ADDC_REG_NUM_BYTE,
            ADDC_ADDR_NUM_BYTE,
            ADDC_REG_ADDR_BYTE,
            ADDC_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "ADDC.W",
        make_math_args_map_w(
            ADDC_REG_REG_WORD,
            ADDC_ADDR_REG_WORD,
            ADDC_REG_NUM_WORD,
            ADDC_ADDR_NUM_WORD,
            ADDC_REG_ADDR_WORD,
            ADDC_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "SUBC.B",
        make_math_args_map_b(
            SUBC_REG_REG_BYTE,
            SUBC_ADDR_REG_BYTE,
            SUBC_REG_NUM_BYTE,
            SUBC_ADDR_NUM_BYTE,
            SUBC_REG_ADDR_BYTE,
            SUBC_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "SUBC.W",
        make_math_args_map_w(
            SUBC_REG_REG_WORD,
            SUBC_ADDR_REG_WORD,
            SUBC_REG_NUM_WORD,
            SUBC_ADDR_NUM_WORD,
            SUBC_REG_ADDR_WORD,
            SUBC_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "MUL.B",
        make_mul_args_map_b(
            MUL_REG_REG_BYTE,
            MUL_ADDR_REG_BYTE,
            MUL_REG_NUM_BYTE,
            MUL_ADDR_NUM_BYTE,
            MUL_REG_ADDR_BYTE,
            MUL_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "MUL.W",
        make_math_args_map_w(
            MUL_REG_REG_WORD,
            MUL_ADDR_REG_WORD,
            MUL_REG_NUM_WORD,
            MUL_ADDR_NUM_WORD,
            MUL_REG_ADDR_WORD,
            MUL_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "DIV.B",
        make_math_args_map_b(
            DIV_REG_REG_BYTE,
            DIV_ADDR_REG_BYTE,
            DIV_REG_NUM_BYTE,
            DIV_ADDR_NUM_BYTE,
            DIV_REG_ADDR_BYTE,
            DIV_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "DIV.W",
        make_math_args_map_w(
            DIV_REG_REG_WORD,
            DIV_ADDR_REG_WORD,
            DIV_REG_NUM_WORD,
            DIV_ADDR_NUM_WORD,
            DIV_REG_ADDR_WORD,
            DIV_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "MULS.B",
        make_mul_args_map_b(
            MULS_REG_REG_BYTE,
            MULS_ADDR_REG_BYTE,
            MULS_REG_NUM_BYTE,
            MULS_ADDR_NUM_BYTE,
            MULS_REG_ADDR_BYTE,
            MULS_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "MULS.W",
        make_math_args_map_w(
            MULS_REG_REG_WORD,
            MULS_ADDR_REG_WORD,
            MULS_REG_NUM_WORD,
            MULS_ADDR_NUM_WORD,
            MULS_REG_ADDR_WORD,
            MULS_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "DIVS.B",
        make_math_args_map_b(
            DIVS_REG_REG_BYTE,
            DIVS_ADDR_REG_BYTE,
            DIVS_REG_NUM_BYTE,
            DIVS_ADDR_NUM_BYTE,
            DIVS_REG_ADDR_BYTE,
            DIVS_ADDR_ADDR_BYTE,
        ),
    );
    map.insert(
        "DIVS.W",
        make_math_args_map_w(
            DIVS_REG_REG_WORD,
            DIVS_ADDR_REG_WORD,
            DIVS_REG_NUM_WORD,
            DIVS_ADDR_NUM_WORD,
            DIVS_REG_ADDR_WORD,
            DIVS_ADDR_ADDR_WORD,
        ),
    );
    map.insert(
        "ASL.B",
        make_logic_args_map_b(ASL_REG_REG_BYTE, ASL_REG_NUM_BYTE, ASL_ADDR_BYTE),
    );
    map.insert(
        "ASL.W",
        make_logic_args_map_w(ASL_REG_REG_WORD, ASL_REG_NUM_WORD, ASL_ADDR_WORD),
    );
    map.insert(
        "ASR.B",
        make_logic_args_map_b(ASR_REG_REG_BYTE, ASR_REG_NUM_BYTE, ASR_ADDR_BYTE),
    );
    map.insert(
        "ASR.W",
        make_logic_args_map_w(ASR_REG_REG_WORD, ASR_REG_NUM_WORD, ASR_ADDR_WORD),
    );
    map.insert(
        "LSR.B",
        make_logic_args_map_b(LSR_REG_REG_BYTE, LSR_REG_NUM_BYTE, LSR_ADDR_BYTE),
    );
    map.insert(
        "LSR.W",
        make_logic_args_map_w(LSR_REG_REG_WORD, LSR_REG_NUM_WORD, LSR_ADDR_WORD),
    );
    map.insert(
        "ROL.B",
        make_logic_args_map_b(ROL_REG_REG_BYTE, ROL_REG_NUM_BYTE, ROL_ADDR_BYTE),
    );
    map.insert(
        "ROL.W",
        make_logic_args_map_w(ROL_REG_REG_WORD, ROL_REG_NUM_WORD, ROL_ADDR_WORD),
    );
    map.insert(
        "ROR.B",
        make_logic_args_map_b(ROR_REG_REG_BYTE, ROR_REG_NUM_BYTE, ROR_ADDR_BYTE),
    );
    map.insert(
        "ROR.W",
        make_logic_args_map_w(ROR_REG_REG_WORD, ROR_REG_NUM_WORD, ROR_ADDR_WORD),
    );
    map.insert(
        "INC.B",
        HashMap::from([
            ("R", INC_REG_BYTE),
            ("I", INC_REG_BYTE),
            ("A", INC_ADDR_BYTE),
        ]),
    );
    map.insert(
        "INC.W",
        HashMap::from([
            ("E", INC_REG_WORD),
            ("I", INC_REG_WORD),
            ("A", INC_ADDR_WORD),
        ]),
    );
    map.insert(
        "DEC.B",
        HashMap::from([
            ("R", DEC_REG_BYTE),
            ("I", DEC_REG_BYTE),
            ("A", DEC_ADDR_BYTE),
        ]),
    );
    map.insert(
        "DEC.W",
        HashMap::from([
            ("E", DEC_REG_WORD),
            ("I", DEC_REG_WORD),
            ("A", DEC_ADDR_WORD),
        ]),
    );
    map.insert("RET", no_args(RET));
    map.insert("RETI", no_args(RETI));
    map.insert("JRF", HashMap::from([("B", JRF_BYTE)]));
    map.insert("JRB", HashMap::from([("B", JRB_BYTE)]));
    map.insert(
        "JMP",
        HashMap::from([("A", JMP_ADDR), ("I", JMP_REG), ("E", JMP_REG)]),
    );
    map.insert(
        "JE",
        HashMap::from([("A", JE_ADDR), ("I", JE_REG), ("E", JE_REG)]),
    );
    map.insert(
        "JNE",
        HashMap::from([("A", JNE_ADDR), ("I", JNE_REG), ("E", JNE_REG)]),
    );
    map.insert(
        "JL",
        HashMap::from([("A", JL_ADDR), ("I", JL_REG), ("E", JL_REG)]),
    );
    map.insert(
        "JG",
        HashMap::from([("A", JG_ADDR), ("I", JG_REG), ("E", JG_REG)]),
    );
    map.insert(
        "JLE",
        HashMap::from([("A", JLE_ADDR), ("I", JLE_REG), ("E", JLE_REG)]),
    );
    map.insert(
        "JGE",
        HashMap::from([("A", JGE_ADDR), ("I", JGE_REG), ("E", JGE_REG)]),
    );
    map.insert(
        "JBC",
        HashMap::from([
            ("IB", JBC_REG_NUM),
            ("EB", JBC_REG_NUM),
            ("IR", JBC_REG_REG),
            ("ER", JBC_REG_REG),
            ("AB", JBC_ADDR_NUM),
            ("AR", JBC_ADDR_REG),
        ]),
    );
    map.insert(
        "JBS",
        HashMap::from([
            ("IB", JBS_REG_NUM),
            ("EB", JBS_REG_NUM),
            ("IR", JBS_REG_REG),
            ("ER", JBS_REG_REG),
            ("AB", JBS_ADDR_NUM),
            ("AR", JBS_ADDR_REG),
        ]),
    );
    map.insert(
        "CALL",
        HashMap::from([("E", CALL_REG), ("I", CALL_REG), ("A", CALL_ADDR)]),
    );
    map.insert(
        "PUSH.B",
        HashMap::from([("R", PUSH_REG_BYTE), ("B", PUSH_NUM_BYTE)]),
    );
    map.insert(
        "PUSH.W",
        HashMap::from([("E", PUSH_REG_WORD), ("W", PUSH_NUM_WORD)]),
    );
    map.insert("POP.B", HashMap::from([("R", POP_REG_BYTE)]));
    map.insert("POP.W", HashMap::from([("E", POP_REG_WORD)]));
    map.insert(
        "SWAP.B",
        HashMap::from([
            ("II", SWAP_REG_REG_BYTE),
            ("RI", SWAP_REG_REG_BYTE),
            ("IR", SWAP_REG_REG_BYTE),
            ("RR", SWAP_REG_REG_BYTE),
        ]),
    );
    map.insert(
        "SWAP.W",
        HashMap::from([
            ("II", SWAP_REG_REG_WORD),
            ("EI", SWAP_REG_REG_WORD),
            ("IE", SWAP_REG_REG_WORD),
            ("EE", SWAP_REG_REG_WORD),
        ]),
    );
    map.insert(
        "MCPY",
        HashMap::from([
            ("AAB", MEM_CPY_ADDR_ADDR_BYTE),
            ("AAR", MEM_CPY_ADDR_ADDR_REG),
            ("AAI", MEM_CPY_ADDR_ADDR_REG),
            ("AEB", MEM_CPY_ADDR_REG_BYTE),
            ("AIB", MEM_CPY_ADDR_REG_BYTE),
            ("IAB", MEM_CPY_REG_ADDR_BYTE),
            ("EAB", MEM_CPY_REG_ADDR_BYTE),
            ("IIB", MEM_CPY_REG_REG_BYTE),
            ("EIB", MEM_CPY_REG_REG_BYTE),
            ("IEB", MEM_CPY_REG_REG_BYTE),
            ("AII", MEM_CPY_ADDR_REG_REG),
            ("III", MEM_CPY_REG_REG_REG),
            ("EII", MEM_CPY_REG_REG_REG),
            ("IEI", MEM_CPY_REG_REG_REG),
            ("EEI", MEM_CPY_REG_REG_REG),
            ("AIR", MEM_CPY_ADDR_REG_REG),
            ("AER", MEM_CPY_ADDR_REG_REG),
            ("IAR", MEM_CPY_REG_ADDR_REG),
            ("EAR", MEM_CPY_REG_ADDR_REG),
            ("IAI", MEM_CPY_REG_ADDR_REG),
            ("EAI", MEM_CPY_REG_ADDR_REG),
            ("IIR", MEM_CPY_REG_REG_REG),
            ("EIR", MEM_CPY_REG_REG_REG),
            ("EER", MEM_CPY_REG_REG_REG),
            ("IER", MEM_CPY_REG_REG_REG),
        ]),
    );
    map.insert(
        "OR.B",
        make_bitwise_args_map_b(OR_REG_REG_BYTE, OR_REG_NUM_BYTE),
    );
    map.insert(
        "OR.W",
        make_bitwise_args_map_w(OR_REG_REG_WORD, OR_REG_NUM_WORD),
    );
    map.insert(
        "XOR.B",
        make_bitwise_args_map_b(XOR_REG_REG_BYTE, XOR_REG_NUM_BYTE),
    );
    map.insert(
        "XOR.W",
        make_bitwise_args_map_w(XOR_REG_REG_WORD, XOR_REG_NUM_WORD),
    );
    map.insert(
        "AND.B",
        make_bitwise_args_map_b(AND_REG_REG_BYTE, AND_REG_NUM_BYTE),
    );
    map.insert(
        "AND.W",
        make_bitwise_args_map_w(AND_REG_REG_WORD, AND_REG_NUM_WORD),
    );
    map.insert(
        "NOT.B",
        HashMap::from([("R", NOT_REG_BYTE), ("I", NOT_REG_BYTE)]),
    );
    map.insert(
        "NOT.W",
        HashMap::from([("E", NOT_REG_WORD), ("I", NOT_REG_WORD)]),
    );
    map
}

fn no_args(op_code: u8) -> HashMap<&'static str, u8> {
    HashMap::from([("", op_code)])
}

fn make_cmp_args_map_b(rr: u8, rn: u8, ra: u8) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IB", rn),
        ("RB", rn),
        ("RI", rr),
        ("IR", rr),
        ("II", rr),
        ("RR", rr),
        ("IA", ra),
        ("RA", ra),
    ])
}

fn make_cmp_args_map_w(rr: u8, rn: u8, ra: u8) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IW", rn),
        ("EW", rn),
        ("EI", rr),
        ("IE", rr),
        ("II", rr),
        ("EE", rr),
        ("IA", ra),
        ("EA", ra),
    ])
}

fn make_bitwise_args_map_b(rr: u8, rn: u8) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IB", rn),
        ("RB", rn),
        ("RI", rr),
        ("IR", rr),
        ("II", rr),
        ("RR", rr),
    ])
}

fn make_bitwise_args_map_w(rr: u8, rn: u8) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IW", rn),
        ("EW", rn),
        ("EI", rr),
        ("IE", rr),
        ("II", rr),
        ("EE", rr),
    ])
}

fn make_logic_args_map_b(rr: u8, rn: u8, a: u8) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IB", rn),
        ("RB", rn),
        ("A", a),
        ("RI", rr),
        ("IR", rr),
        ("II", rr),
        ("RR", rr),
    ])
}

fn make_logic_args_map_w(rr: u8, rn: u8, a: u8) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IW", rn),
        ("EW", rn),
        ("A", a),
        ("EI", rr),
        ("IE", rr),
        ("II", rr),
        ("EE", rr),
    ])
}

fn make_math_args_map_b(
    rr: u8,
    ar: u8,
    rn: u8,
    an: u8,
    ra: u8,
    aa: u8,
) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IB", rn),
        ("RB", rn),
        ("IA", ra),
        ("RA", ra),
        ("AI", ar),
        ("AR", ar),
        ("AB", an),
        ("RR", rr),
        ("RI", rr),
        ("IR", rr),
        ("II", rr),
        ("AA", aa),
    ])
}

fn make_mul_args_map_b(
    rr: u8,
    ar: u8,
    rn: u8,
    an: u8,
    ra: u8,
    aa: u8,
) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IB", rn),
        ("EB", rn),
        ("IA", ra),
        ("EA", ra),
        ("AI", ar),
        ("AR", ar),
        ("AB", an),
        ("ER", rr),
        ("EI", rr),
        ("IR", rr),
        ("II", rr),
        ("AA", aa),
    ])
}

fn make_math_args_map_w(
    rr: u8,
    ar: u8,
    rn: u8,
    an: u8,
    ra: u8,
    aa: u8,
) -> HashMap<&'static str, u8> {
    HashMap::from([
        ("IW", rn),
        ("EW", rn),
        ("IA", ra),
        ("EA", ra),
        ("AI", ar),
        ("AE", ar),
        ("AW", an),
        ("EE", rr),
        ("EI", rr),
        ("IE", rr),
        ("II", rr),
        ("AA", aa),
    ])
}

#[cfg(test)]
mod test {
    use crate::arg_patterns::ARG_MATCHES;
    use maikor_platform::{op_desc, ops};
    use std::collections::HashMap;

    #[test]
    fn check_all_ops_found() {
        let ops = ops::ALL.to_vec();
        for (op_name, matches) in ARG_MATCHES.iter() {
            for (key, op_code) in matches {
                if !ops.iter().any(|val| val == op_code) {
                    panic!("Unknown op found in ARG_MATCHES op code: {:02X}, op name: {op_name}, arg pattern: {key}", op_code);
                }
            }
        }
    }

    #[test]
    fn check_all_ops_used() {
        let mut ops = ops::ALL.to_vec();
        for matches in ARG_MATCHES.values() {
            for op_code in matches.values() {
                if let Some(idx) = ops.iter().position(|val| val == op_code) {
                    ops.remove(idx);
                }
            }
        }
        if !ops.is_empty() {
            panic!(
                "Following ops not in ARG_MATCHES: \n{:?}",
                ops.iter()
                    .map(|op| op_desc(*op).unwrap())
                    .collect::<Vec<&'static str>>()
            );
        }
    }

    #[test]
    fn check_no_duplicate_arg_patterns() {
        for (op_name, matches) in ARG_MATCHES.iter() {
            check_arg_patterns(op_name, matches);
        }
    }

    fn check_arg_patterns(op_name: &str, matches: &HashMap<&str, u8>) {
        let mut patterns: Vec<String> = vec![];
        for (key, code) in matches {
            let id = format!("{}={:02X}", key, code);
            assert!(!patterns.contains(&id), "{op_name} contains multiple {id}",);
            patterns.push(id);
        }
    }
}
