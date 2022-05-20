mod arg_matching;
mod arg_patterns;
mod interpreter;
mod ops;
mod parsers;

use crate::arg_matching::{arg_list_to_letters, get_op_code};
use crate::interpreter::interpret_line;
use crate::parsers::parse_argument;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Line was empty (internal parser error)")]
    EmptyLine,
    #[error("Unable to parse line {0}: {1} ({2})")]
    General(usize, String, String),
    #[error("Invalid Address format {2}: {1} on line {0}, must be $x0 - $xFFFF")]
    AddressHexFormat(usize, String, String),
    #[error("Invalid Address format {2}: {1} on line {0}, must be $0 - $65535")]
    AddressNumFormat(usize, String, String),
    #[error(
        "Address out outside of valid range {1} on line {0}, must be less than 65535 or xFFFF"
    )]
    AddressTooBig(usize, String),
    #[error("Invalid Number literal format {2}: {1} on line {0}, must be 0 - 65535")]
    NumberFormat(usize, String, String),
    #[error("Invalid Number literal format {2}: {1} on line {0}, must be x0 - xFFFF")]
    NumberHexFormat(usize, String, String),
    #[error("Number literal out outside of valid range {1} on line {0}, must be less than 65535 or xFFFF")]
    NumberTooBig(usize, String),
    #[error("Register has invalid format {1} on line {0}, expected {2}")]
    InvalidRegister(usize, String, String),
    #[error("Invalid Number literal format {2}, {1} on line {0}, must be -32768 to 32767")]
    SignedNumberNumFormat(usize, String, String),
    #[error("Invalid Number literal format {1} on line {0}, must be -32768 to 32767")]
    SignedNumberNumRange(usize, String),
    #[error("This instruction only supports byte (0-255), was {1} on line {0}")]
    NumberMustBeByte(usize, String),
    #[error("Instruction unknown/unsupported: {1} {1:02X} on line {0}")]
    InvalidOpCode(usize, u8),
    #[error("Arguments {1} don't match instruction {2} (line {0}), supported: {3}")]
    InvalidArguments(usize, String, String, String),
    #[error("{1} (line {0}) requires arguments, supported: {2}")]
    MissingArguments(usize, String, String),
    #[error("No op found named '{1}', maybe you're missing the size? ('.B' or '.W') on line {0}")]
    InvalidOpName(usize, String),
    #[error(
        "Invalid character literal {1}, must be one ASCII character in single quotes on line {0}"
    )]
    InvalidCharacter(usize, String),
    #[error("Couldn't parse number or register for offset {1} on line {0}")]
    InvalidOffset(usize, String),
}

impl ParseError {
    fn num_to_addr(self) -> Self {
        match self {
            ParseError::NumberFormat(line_num, msg, err) => {
                ParseError::AddressNumFormat(line_num, msg, err)
            }
            ParseError::NumberHexFormat(line_num, msg, err) => {
                ParseError::AddressHexFormat(line_num, msg, err)
            }
            ParseError::NumberTooBig(line_num, msg) => ParseError::AddressTooBig(line_num, msg),
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line {
    pub num: usize,
    pub original: String,
    pub label: Option<String>,
    pub command: Option<(String, Vec<String>)>,
}

impl Line {
    fn new(num: usize, original: String) -> Self {
        Self {
            num,
            original,
            label: None,
            command: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program {
    pub lines: Vec<ParsedLine>,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParsedLine {
    pub line: Line,
    pub bytes: Vec<u8>,
}

pub fn parse_program(lines: &[&str]) -> Result<Program, ParseError> {
    let mut output = vec![];
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let (trimmed, _) = trimmed.split_once('#').unwrap_or((trimmed, ""));
        if !trimmed.is_empty() {
            let line = interpret_line(idx, line)?;
            output.push(parse_line(line)?);
        }
    }
    let bytes = output.iter().flat_map(|line| line.bytes.clone()).collect();
    let program = Program {
        lines: output,
        bytes,
    };
    Ok(program)
}

fn parse_line(line: Line) -> Result<ParsedLine, ParseError> {
    let mut bytes = vec![];
    if let Some((op, args)) = &line.command {
        let command = op.to_ascii_uppercase();
        let mut arguments = vec![];
        let expects_bytes = ops::expects_bytes(&command);
        for arg in args {
            let arg_token = parse_argument(line.num, arg)?;
            arguments.push(arg_token.to_argument(expects_bytes));
        }
        let pattern = arg_list_to_letters(&arguments);
        bytes.push(get_op_code(line.num, &command, &pattern)?);
        for arg in arguments {
            bytes.extend_from_slice(&arg.to_bytes());
        }
    }
    Ok(ParsedLine { line, bytes })
}

pub fn parse_line_from_str(text: &str) -> Result<ParsedLine, ParseError> {
    let line = interpret_line(0, text)?;
    parse_line(line)
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_platform::ops::{
        ADD_REG_NUM_BYTE, CMP_REG_NUM_BYTE, INC_REG_BYTE, INC_REG_WORD, JE_ADDR,
    };
    use maikor_platform::registers::id;
    use maikor_platform::registers::id::AL;

    #[test]
    fn line_test() {
        assert_eq!(
            parse_line_from_str("inc.w bx").unwrap(),
            ParsedLine {
                line: Line {
                    num: 0,
                    original: "inc.w bx".to_string(),
                    label: None,
                    command: Some(("inc.w".to_string(), vec!["bx".to_string()])),
                },
                bytes: vec![INC_REG_WORD, id::BX as u8],
            }
        );
        assert_eq!(
            parse_line_from_str("add.b al, 30").unwrap(),
            ParsedLine {
                line: Line {
                    num: 0,
                    original: "add.b al, 30".to_string(),
                    label: None,
                    command: Some((
                        "add.b".to_string(),
                        vec!["al".to_string(), "30".to_string()]
                    )),
                },
                bytes: vec![ADD_REG_NUM_BYTE, id::AL as u8, 30],
            }
        );

        assert!(parse_line_from_str("inc al").is_err());
    }

    #[test]
    fn basic_test() {
        let lines = vec!["# test program", "INC.B AL", "CMP.B AL, 1", "JE $50"];
        let output = parse_program(&lines).unwrap();
        assert_eq!(output.lines.len(), 3);
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
                50,
            ]
        );
    }
}
