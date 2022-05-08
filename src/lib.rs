pub mod arg_matches;
pub mod line;
pub mod registers;
pub mod tokens;

use crate::arg_matches::get_op_code;
use crate::line::parse_line;
use crate::tokens::{to_args_str, Token};
use crate::ParserError::{EmptyLine, General, Language};
use maikor_language::LangError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Line was empty (internal parser error)")]
    EmptyLine,
    #[error("")]
    Language(#[source] LangError),
    #[error("Unable to parse: '{0}'")]
    General(String),
    #[error("Invalid address: '{0}'")]
    InvalidAddress(String),
    #[error("Invalid address: '{0}'")]
    InvalidNumber(String),
    #[error("Only AX-DX can be indirect, was {0}")]
    NotExtRegister(String),
    #[error("Invalid register: '{0}'")]
    InvalidRegister(String),
}

impl From<LangError> for ParserError {
    fn from(err: LangError) -> Self {
        Language(err)
    }
}

pub struct ParserOutput {
    pub bytes: Vec<u8>,
    pub op_count: usize,
}

pub fn parse_lines(lines: &[&str]) -> Result<ParserOutput, ParserError> {
    let mut op_count = 0;
    let mut bytes = vec![];

    for line in lines {
        let trimmed = line.trim();
        if !trimmed.starts_with('#') && !trimmed.is_empty() {
            let (op, args) = parse_line(trimmed)?;
            op_count += 1;
            bytes.push(op);
            bytes.extend_from_slice(&args);
        }
    }

    Ok(ParserOutput { bytes, op_count })
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_language::ops::{CMP_REG_NUM_BYTE, INC_REG_BYTE, JE_ADDR};
    use maikor_language::registers::offset::AL;

    #[test]
    fn basic_test() {
        let lines = vec!["# test program", "INC.B AL", "CMP.B AL 1", "JE $50"];
        let output = parse_lines(&lines).unwrap();
        assert_eq!(output.op_count, 3);
        assert_eq!(
            output.bytes,
            vec![
                INC_REG_BYTE,
                AL as u8,
                CMP_REG_NUM_BYTE,
                AL as u8,
                1,
                JE_ADDR,
                0,
                50
            ]
        );
    }
}
