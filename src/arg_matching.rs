use crate::arg_patterns::ARG_MATCHES;
use crate::ParseError;
use crate::ParseError::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Argument {
    Address(u16),
    Register(u8),
    ExtReg(u8),
    IndirectReg(u8, Option<u8>, Option<u16>),
    Word(u16),
    Byte(u8),
}

impl Argument {
    fn letter(&self) -> char {
        match self {
            Argument::Address(_) => 'A',
            Argument::Register(_) => 'R',
            Argument::ExtReg(_) => 'E',
            Argument::IndirectReg(_, _, _) => 'I',
            Argument::Word(_) => 'W',
            Argument::Byte(_) => 'B',
        }
    }

    pub fn to_offset_bytes(&self) -> Vec<u8> {
        if let Argument::IndirectReg(_, offset_reg, offset_num) = self {
            let mut output = vec![];
            if let Some(reg) = offset_reg {
                output.push(*reg);
            }
            if let Some(num) = offset_num {
                output.extend_from_slice(&num.to_be_bytes());
            }
            output
        } else {
            vec![]
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Argument::Address(addr) => addr.to_be_bytes().to_vec(),
            Argument::Register(reg) => vec![*reg],
            Argument::ExtReg(reg) => vec![*reg],
            Argument::IndirectReg(reg, _, _) => vec![*reg],
            Argument::Word(word) => word.to_be_bytes().to_vec(),
            Argument::Byte(byte) => vec![*byte],
        }
    }
}

pub fn arg_list_to_letters(args: &[Argument]) -> String {
    let mut output = String::new();
    for arg in args {
        output.push(arg.letter());
    }
    output
}

pub fn get_op_code(line_num: usize, op_name: &str, pattern: &str) -> Result<u8, ParseError> {
    if let Some(map) = ARG_MATCHES.get(op_name) {
        if let Some(op_code) = map.get(pattern) {
            Ok(*op_code)
        } else {
            let options: Vec<&&str> = map.keys().collect();
            let options_text = format!("{:?}", options);
            if pattern.is_empty() {
                Err(MissingArguments(
                    line_num,
                    op_name.to_string(),
                    options_text,
                ))
            } else {
                Err(InvalidArguments(
                    line_num,
                    pattern.to_string(),
                    op_name.to_string(),
                    options_text,
                ))
            }
        }
    } else {
        Err(InvalidOpName(line_num, op_name.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::Argument::*;
    use maikor_platform::op_params::*;

    #[test]
    fn test_to_bytes() {
        assert_eq!(Address(0).to_bytes(), vec![0, 0]);
        assert_eq!(Address(1).to_bytes(), vec![0, 1]);
        assert_eq!(Address(256).to_bytes(), vec![1, 0]);
        assert_eq!(Address(5311).to_bytes(), vec![20, 191]);
        assert_eq!(Address(0).to_bytes(), vec![0, 0]);
        assert_eq!(Address(1).to_bytes(), vec![0, 1]);
        assert_eq!(Address(256).to_bytes(), vec![1, 0]);
        assert_eq!(Address(5311).to_bytes(), vec![20, 191]);
        assert_eq!(Byte(0).to_bytes(), vec![0]);
        assert_eq!(Byte(1).to_bytes(), vec![1]);
        assert_eq!(Word(0).to_bytes(), vec![0, 0]);
        assert_eq!(Word(1).to_bytes(), vec![0, 1]);
        assert_eq!(Word(256).to_bytes(), vec![1, 0]);
        assert_eq!(Register(0).to_bytes(), vec![0]);
        assert_eq!(Register(2).to_bytes(), vec![2]);
        assert_eq!(Register(6 | PRE_DEC).to_bytes(), vec![6 | PRE_DEC]);
        assert_eq!(
            IndirectReg(9 | IND_OFFSET_REG, Some(2), None).to_bytes(),
            vec![9 | IND_OFFSET_REG]
        );
        assert_eq!(
            IndirectReg(9 | IND_OFFSET_REG, Some(2), None).to_offset_bytes(),
            vec![2]
        );
        assert_eq!(
            IndirectReg(10 | IND_OFFSET_EXT_REG, Some(11), None).to_bytes(),
            vec![10 | IND_OFFSET_EXT_REG]
        );
        assert_eq!(
            IndirectReg(10 | IND_OFFSET_EXT_REG, Some(11), None).to_offset_bytes(),
            vec![11]
        );
        assert_eq!(
            IndirectReg(9 | IND_OFFSET_NUM, None, Some(15)).to_bytes(),
            vec![9 | IND_OFFSET_NUM]
        );
        assert_eq!(
            IndirectReg(9 | IND_OFFSET_NUM, None, Some(15)).to_offset_bytes(),
            vec![0, 15]
        );
        assert_eq!(
            IndirectReg(9 | IND_OFFSET_NUM, None, Some(567)).to_bytes(),
            vec![9 | IND_OFFSET_NUM]
        );
        assert_eq!(
            IndirectReg(9 | IND_OFFSET_NUM, None, Some(567)).to_offset_bytes(),
            vec![2, 55]
        );
        assert_eq!(ExtReg(9).to_bytes(), vec![9]);
        assert_eq!(ExtReg(9 | PRE_DEC).to_bytes(), vec![9 | PRE_DEC]);
    }
}
