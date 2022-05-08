use crate::ParserError::{General, InvalidAddress, InvalidNumber, NotExtRegister};
use crate::{registers, ParserError};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    //(?i) = case insensitive
    //[[:xdigit:]] = hex digit
    static ref ADDRESS: Regex = Regex::new(r"^\$\d{1,5}").unwrap();
    static ref ADDRESS_HEX: Regex = Regex::new(r"(?i)^\$x[[:xdigit:]]{1,4}").unwrap();
    static ref REGISTER: Regex = Regex::new(r"(?i)^[+-]?([A-D][LH]|FLG)[+-]?").unwrap();
    static ref EXT_REGISTER: Regex = Regex::new(r"(?i)^[+-]?\(?([A-D][X])\)?[+-]?").unwrap();
    static ref INDIRECT_REGISTER: Regex = Regex::new(r"(?i)^[+-]?\(([A-D][X])\)[+-]?").unwrap();
    static ref WORD_HEX: Regex = Regex::new(r"(?i)^x[[:xdigit:]]{1,4}").unwrap();
    static ref BYTE_HEX: Regex = Regex::new(r"(?i)^x[[:xdigit:]]{1,2}").unwrap();
    static ref WORD_NUM: Regex = Regex::new(r"^\d{1,5}").unwrap();
    static ref BYTE_NUM: Regex = Regex::new(r"^\d{1,3}").unwrap();

    static ref INVALID_INDIRECT_REG: Regex = Regex::new(r"(?i)^\([A-D][LH]\)").unwrap();
    static ref INVALID_NUMBER: Regex = Regex::new(r"^\d{6,}").unwrap();
    static ref INVALID_ADDRESS: Regex = Regex::new(r"^x\d{6,}").unwrap();
    static ref INVALID_HEX_NUMBER: Regex = Regex::new(r"(?i)^x[[:xdigit:]]{5,}").unwrap();
    static ref INVALID_HEX_ADDRESS: Regex = Regex::new(r"(?i)^$x[[:xdigit:]]{5,}").unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Byte(u8),
    Word(u16),
    Address(u16),
    Register(u8),
    ExtRegister(u8),
    IndirectRegister(u8),
}

impl From<&Token> for Vec<u8> {
    fn from(token: &Token) -> Self {
        match token {
            Token::Byte(b) => vec![*b],
            Token::Word(w) => w.to_be_bytes().to_vec(),
            Token::Address(a) => a.to_be_bytes().to_vec(),
            Token::Register(r) => vec![*r],
            Token::ExtRegister(r) => vec![*r],
            Token::IndirectRegister(r) => vec![*r],
        }
    }
}

pub fn check_known_invalid_values(text: &str) -> Result<(), ParserError> {
    if INVALID_ADDRESS.is_match(text) {
        return Err(InvalidAddress(format!(
            "Must be less than 65535 | xFFFF, was {}",
            text
        )));
    }
    if INVALID_NUMBER.is_match(text) {
        return Err(InvalidNumber(format!(
            "Must be less than 65535 | xFFFF, was {}",
            text
        )));
    }
    if INVALID_INDIRECT_REG.is_match(text) {
        return Err(NotExtRegister(text.to_string()));
    }
    Ok(())
}

pub fn create_token(text: &str, promote_bytes: bool) -> Result<Token, ParserError> {
    check_known_invalid_values(text)?;
    if ADDRESS.is_match(text) {
        let addr_txt: String = text.chars().skip(1).collect();
        return if let Ok(addr) = u16::from_str(&addr_txt) {
            Ok(Token::Address(addr))
        } else {
            Err(InvalidAddress(text.to_string()))
        };
    } else if ADDRESS_HEX.is_match(text) {
        let addr_txt: String = text.chars().skip(2).collect();
        return if let Ok(addr) = u16::from_str_radix(&addr_txt, 16) {
            Ok(Token::Address(addr))
        } else {
            Err(InvalidAddress(text.to_string()))
        };
    }
    if INDIRECT_REGISTER.is_match(text) {
        let reg = registers::parse_register(text)?;
        return Ok(Token::IndirectRegister(reg));
    }
    if REGISTER.is_match(text) {
        let reg = registers::parse_register(text)?;
        return Ok(Token::Register(reg));
    }
    if EXT_REGISTER.is_match(text) {
        let reg = registers::parse_register(text)?;
        return Ok(Token::ExtRegister(reg));
    }
    if !promote_bytes {
        if BYTE_NUM.is_match(text) {
            if let Ok(num) = u8::from_str(text) {
                return Ok(Token::Byte(num));
            }
        }
        if BYTE_HEX.is_match(text) {
            let num_txt: String = text.chars().skip(1).collect();
            if let Ok(num) = u8::from_str_radix(&num_txt, 16) {
                return Ok(Token::Byte(num));
            }
        }
    }
    if WORD_NUM.is_match(text) {
        return if let Ok(num) = u16::from_str(text) {
            Ok(Token::Word(num))
        } else {
            Err(InvalidNumber(text.to_string()))
        };
    }
    if WORD_HEX.is_match(text) {
        let num_txt: String = text.chars().skip(1).collect();
        return if let Ok(num) = u16::from_str_radix(&num_txt, 16) {
            Ok(Token::Word(num))
        } else {
            Err(InvalidNumber(text.to_string()))
        };
    }
    Err(General(text.to_string()))
}

pub fn to_args_str(list: &[Token]) -> String {
    let mut output = String::new();
    for token in list {
        match token {
            Token::Byte(_) => output.push('B'),
            Token::Word(_) => output.push('W'),
            Token::Address(_) => output.push('A'),
            Token::Register(_) => output.push('R'),
            Token::ExtRegister(_) => output.push('E'),
            Token::IndirectRegister(_) => output.push('I'),
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_language::op_params::{PRE_DEC, PRE_INC};
    use maikor_language::registers::offset;
    use crate::Token::*;

    #[test]
    fn test_addresses() {
        assert_eq!(create_token("$1", false).unwrap(), Address(1));
        assert_eq!(create_token("$1234", false).unwrap(), Address(1234));
    }

    #[test]
    fn test_hex_addresses() {
        assert_eq!(create_token("$x1",false).unwrap(), Address(1));
        assert_eq!(create_token("$x4d2",false).unwrap(), Address(1234));
    }

    #[test]
    fn test_registers() {
        assert_eq!(create_token("AL",false).unwrap(), Register(offset::AL as u8));
        assert_eq!(create_token("CL",false).unwrap(), Register(offset::CL as u8));
        assert_eq!(
            create_token("-CL",false).unwrap(),
            Register(offset::CL as u8 | PRE_DEC)
        );
    }

    #[test]
    fn test_ext_registers() {
        assert_eq!(
            create_token("+AX",false).unwrap(),
            ExtRegister(offset::AX as u8 + PRE_INC)
        );
        assert_eq!(
            create_token("BX",false).unwrap(),
            ExtRegister(offset::BX as u8)
        );
    }

    #[test]
    fn test_byte() {
        assert_eq!(create_token("2", true).unwrap(), Word(2));
        assert_eq!(create_token("189", true).unwrap(), Word(189));
        assert_eq!(create_token("2", false).unwrap(), Byte(2));
        assert_eq!(create_token("189", false).unwrap(), Byte(189));
    }

    #[test]
    fn test_hex_byte() {
        assert_eq!(create_token("x2", true).unwrap(), Word(2));
        assert_eq!(create_token("xbD", true).unwrap(), Word(189));
        assert_eq!(create_token("x2", false).unwrap(), Byte(2));
        assert_eq!(create_token("xbD", false).unwrap(), Byte(189));
    }

    #[test]
    fn test_word() {
        assert_eq!(create_token("256", false).unwrap(), Word(256));
        assert_eq!(create_token("12045", false).unwrap(), Word(12045));
        assert_eq!(create_token("256", true).unwrap(), Word(256));
        assert_eq!(create_token("12045", true).unwrap(), Word(12045));
    }

    #[test]
    fn test_hex_word() {
        assert_eq!(create_token("x100", true).unwrap(), Word(256));
        assert_eq!(create_token("x2f0d", true).unwrap(), Word(12045));
        assert_eq!(create_token("x100",false).unwrap(), Word(256));
        assert_eq!(create_token("x2f0d",false).unwrap(), Word(12045));
    }

    #[test]
    fn check_to_args_str() {
        assert_eq!("", to_args_str(&[]));
        assert_eq!("AI", to_args_str(&[Address(0), IndirectRegister(0)]));
        assert_eq!(
            "EAIWBR",
            to_args_str(&[
                ExtRegister(0),
                Address(0),
                IndirectRegister(0),
                Word(0),
                Byte(0),
                Register(0)
            ])
        );
    }
}
