use crate::arg_matching::Argument;
use crate::ParseError;
use crate::ParseError::*;
use maikor_platform::op_params::{RegisterPPID, ID, INDIRECT, PP};
use maikor_platform::registers::id;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ArgToken {
    Address(u16),
    Register(u8, Option<u8>, Option<u16>),
    Number(u16),
}

impl ArgToken {
    pub fn to_argument(self, convert_word_to_byte: bool) -> Argument {
        match self {
            ArgToken::Address(addr) => Argument::Address(addr),
            ArgToken::Register(op, offset_reg, offset_num) => {
                if id::size((op & 0x0F) as usize) == 1 {
                    Argument::Register(op)
                } else if op & INDIRECT == 0 {
                    Argument::ExtReg(op)
                } else {
                    Argument::IndirectReg(op, offset_reg, offset_num)
                }
            }
            ArgToken::Number(num) => {
                if convert_word_to_byte && num < 256 {
                    Argument::Byte(num as u8)
                } else {
                    Argument::Word(num)
                }
            }
        }
    }
}

pub fn parse_argument(arg: &str) -> Result<ArgToken, ParseError> {
    let trimmed = arg.trim_matches(|c: char| c == ',' || c.is_whitespace());
    return if trimmed.starts_with('$') {
        match detect_num(arg, trimmed.trim_start_matches('$')) {
            Ok(addr) => {
                if let Some(addr) = addr {
                    Ok(ArgToken::Address(addr))
                } else {
                    Err(GeneralArg(
                        arg.to_string(),
                        String::from("No address after $"),
                    ))
                }
            }
            Err(err) => Err(err.num_to_addr()),
        }
    } else {
        match parse_register(trimmed) {
            Ok(reg) => Ok(reg),
            Err(reg_err) => {
                if let Some(num) = detect_num(arg, trimmed)? {
                    Ok(ArgToken::Number(num))
                } else {
                    Err(reg_err)
                }
            }
        }
    };
}

fn parse_register(reg: &str) -> Result<ArgToken, ParseError> {
    let remaining: String = reg.chars().filter(|c| !c.is_whitespace()).collect();
    let (ppid, remaining) = detect_ppid(&remaining);
    let (is_indirect, remaining) = detect_indirect(reg, remaining)?;
    if let Some((dst, offset)) = remaining.split_once(|c| c == '+') {
        if ppid.is_some() {
            return Err(InvalidRegister(
                reg.to_string(),
                String::from("Can't use PPID and offset"),
            ));
        }
        let dst = detect_register(reg, dst)?;
        let offset = detect_offset(reg, offset)?;
        let meta: u8 = RegisterPPID::new(
            is_indirect,
            offset.reg.is_some(),
            offset.num.is_some(),
            offset.ext_reg.is_some(),
            ppid,
        )
        .into();
        Ok(ArgToken::Register(dst + meta, offset.reg(), offset.num))
    } else {
        let reg = detect_register(reg, remaining)?;
        let meta: u8 = RegisterPPID::new(is_indirect, false, false, false, ppid).into();
        Ok(ArgToken::Register(reg + meta, None, None))
    }
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct Offset {
    reg: Option<u8>,
    ext_reg: Option<u8>,
    num: Option<u16>,
}

impl Offset {
    pub fn new(reg: Option<u8>, ext_reg: Option<u8>, num: Option<u16>) -> Self {
        Self { reg, ext_reg, num }
    }
}

impl Offset {
    pub fn reg(&self) -> Option<u8> {
        if self.reg.is_some() {
            self.reg
        } else {
            self.ext_reg
        }
    }
}

fn detect_offset(original: &str, offset: &str) -> Result<Offset, ParseError> {
    let num_result = detect_num(original, offset);
    let reg_result = detect_register(original, offset);
    if let Ok(reg) = reg_result {
        if id::size(reg as usize) == 1 {
            Ok(Offset::new(Some(reg), None, None))
        } else {
            Ok(Offset::new(None, Some(reg), None))
        }
    } else if let Ok(Some(num)) = num_result {
        Ok(Offset::new(None, None, Some(num)))
    } else {
        Err(InvalidOffset(offset.to_string()))
    }
}

fn detect_register(original: &str, remaining: &str) -> Result<u8, ParseError> {
    match id::from_name(&remaining.to_ascii_uppercase()) {
        Ok(id) => Ok(id as u8),
        Err(err) => Err(InvalidRegister(original.to_string(), err.to_string())),
    }
}

fn detect_indirect<'a>(original: &str, remaining: &'a str) -> Result<(bool, &'a str), ParseError> {
    if remaining.starts_with('(') {
        if remaining.ends_with(')') {
            Ok((true, remaining.trim_matches(|c| c == '(' || c == ')')))
        } else {
            Err(InvalidRegister(
                original.to_string(),
                String::from("')' at end, as '(' was found at start"),
            ))
        }
    } else {
        Ok((false, remaining))
    }
}

fn detect_num(original: &str, remaining: &str) -> Result<Option<u16>, ParseError> {
    if remaining.starts_with('\'') && remaining.ends_with('\'') {
        if remaining.chars().count() == 3 {
            if let Some(chr) = remaining.chars().nth(1) {
                if chr.is_ascii() {
                    return Ok(Some(chr as u8 as u16));
                }
            }
        } else if remaining == "'\''" {
            return Ok(Some(39)); //ASCII ' char
        }
        Err(InvalidCharacter(remaining.to_string()))
    } else if remaining.starts_with('x') {
        match usize::from_str_radix(remaining.trim_start_matches('x'), 16) {
            Ok(num) => {
                if num <= u16::MAX as usize {
                    Ok(Some(num as u16))
                } else {
                    Err(NumberTooBig(original.to_string()))
                }
            }
            Err(err) => Err(NumberHexFormat(original.to_string(), err.to_string())),
        }
    } else if remaining.starts_with('b') {
        match usize::from_str_radix(remaining.trim_start_matches('b'), 2) {
            Ok(num) => {
                if num <= u16::MAX as usize {
                    Ok(Some(num as u16))
                } else {
                    Err(NumberTooBig(original.to_string()))
                }
            }
            Err(err) => Err(NumberFormat(original.to_string(), err.to_string())),
        }
    } else if remaining.starts_with('-') {
        match remaining.parse::<isize>() {
            Ok(num) => {
                if num >= i16::MIN as isize && num <= i16::MAX as isize {
                    Ok(Some(num as i16 as u16))
                } else {
                    Err(SignedNumberNumRange(original.to_string()))
                }
            }
            Err(err) => Err(SignedNumberNumFormat(original.to_string(), err.to_string())),
        }
    } else if remaining.chars().all(|c| c.is_digit(10)) {
        match remaining.parse::<usize>() {
            Ok(num) => {
                if num <= u16::MAX as usize {
                    Ok(Some(num as u16))
                } else {
                    Err(NumberTooBig(original.to_string()))
                }
            }
            Err(err) => Err(NumberFormat(original.to_string(), err.to_string())),
        }
    } else {
        Ok(None)
    }
}

fn detect_ppid(reg: &str) -> (Option<(PP, ID)>, &str) {
    if reg.starts_with('-') {
        return (Some((PP::Pre, ID::Dec)), reg.trim_start_matches('-'));
    } else if reg.starts_with('+') {
        return (Some((PP::Pre, ID::Inc)), reg.trim_start_matches('+'));
    } else if reg.ends_with('-') {
        return (Some((PP::Post, ID::Dec)), reg.trim_end_matches('-'));
    } else if reg.ends_with('+') {
        return (Some((PP::Post, ID::Inc)), reg.trim_end_matches('+'));
    } else {
        (None, reg)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::ArgToken::*;
    use maikor_platform::op_params::{
        INDIRECT, IND_OFFSET_EXT_REG, IND_OFFSET_NUM, IND_OFFSET_REG, IND_POST_INC, IND_PRE_DEC,
        POST_INC, PRE_DEC,
    };

    #[test]
    fn test_conversion() {
        assert_eq!(Number(0).to_argument(false), Argument::Word(0));
        assert_eq!(Number(0).to_argument(true), Argument::Byte(0));
        assert_eq!(Number(100).to_argument(false), Argument::Word(100));
        assert_eq!(Number(100).to_argument(true), Argument::Byte(100));
        assert_eq!(Number(300).to_argument(false), Argument::Word(300));
        assert_eq!(Number(300).to_argument(true), Argument::Word(300));
        assert_eq!(Number(1000).to_argument(false), Argument::Word(1000));
        assert_eq!(Number(1000).to_argument(true), Argument::Word(1000));
        assert_eq!(Address(0).to_argument(false), Argument::Address(0));
        assert_eq!(Address(100).to_argument(false), Argument::Address(100));
        assert_eq!(Address(1000).to_argument(false), Argument::Address(1000));
        assert_eq!(Address(10000).to_argument(false), Argument::Address(10000));
        assert_eq!(Address(0).to_argument(true), Argument::Address(0));
        assert_eq!(Address(100).to_argument(true), Argument::Address(100));
        assert_eq!(Address(1000).to_argument(true), Argument::Address(1000));
        assert_eq!(Address(10000).to_argument(true), Argument::Address(10000));
        assert_eq!(
            Register(0, None, None).to_argument(false),
            Argument::Register(0)
        );
        assert_eq!(
            Register(1, None, None).to_argument(false),
            Argument::Register(1)
        );
        assert_eq!(
            Register(8, None, None).to_argument(false),
            Argument::Register(8)
        );
        assert_eq!(
            Register(9, None, None).to_argument(false),
            Argument::ExtReg(9)
        );
        assert_eq!(
            Register(12, None, None).to_argument(false),
            Argument::ExtReg(12)
        );
        assert_eq!(
            Register(9 | INDIRECT, None, None).to_argument(false),
            Argument::IndirectReg(9 | INDIRECT, None, None)
        );
        assert_eq!(
            Register(9 | INDIRECT, None, None).to_argument(false),
            Argument::IndirectReg(9 | INDIRECT, None, None)
        );
        assert_eq!(
            Register(9 | INDIRECT, None, Some(15)).to_argument(false),
            Argument::IndirectReg(9 | INDIRECT, None, Some(15))
        );
        assert_eq!(
            Register(9 | INDIRECT, Some(1), None).to_argument(false),
            Argument::IndirectReg(9 | INDIRECT, Some(1), None)
        );
    }

    #[test]
    fn test_parse_argument() {
        assert_eq!(parse_argument("605").unwrap(), Number(605));
        assert_eq!(parse_argument("xF11").unwrap(), Number(3857));
        assert_eq!(parse_argument("$100").unwrap(), Address(100));
        assert_eq!(parse_argument("$xF").unwrap(), Address(15));
        assert_eq!(parse_argument("aL").unwrap(), Register(1, None, None));
        assert_eq!(
            parse_argument("(Bx)").unwrap(),
            Register(10 | INDIRECT, None, None)
        );
        assert_eq!(
            parse_argument("-ch").unwrap(),
            Register(4 | PRE_DEC, None, None)
        );
        assert_eq!(
            parse_argument("(dx)+").unwrap(),
            Register(12 | IND_POST_INC, None, None)
        );
        assert_eq!(
            parse_argument("(ax+563)").unwrap(),
            Register(9 | IND_OFFSET_NUM, None, Some(563))
        );
        assert_eq!(
            parse_argument("(ax+dh)").unwrap(),
            Register(9 | IND_OFFSET_REG, Some(6), None)
        );
        assert_eq!(
            parse_argument("(ax+bx)").unwrap(),
            Register(9 | IND_OFFSET_EXT_REG, Some(10), None)
        );

        assert!(parse_argument("a").is_err());
        assert!(parse_argument("78021").is_err());
        assert!(parse_argument("xFFFF1").is_err());
        assert!(parse_argument("$121231").is_err());
        assert!(parse_argument("(dx").is_err());
        assert!(parse_argument("(dx+141351)").is_err());
        assert!(parse_argument("(dx+a)").is_err());
        assert!(parse_argument("(-dx+a)").is_err());
        assert!(parse_argument("((dx)+al)").is_err());
        assert!(parse_argument("(dx+10)-").is_err());
    }

    #[test]
    fn test_register() {
        assert_eq!(parse_register("AH ").unwrap(), (Register(0, None, None)));
        assert_eq!(parse_register("AX").unwrap(), (Register(9, None, None)));
        assert_eq!(
            parse_register("(AX )").unwrap(),
            (Register(9 | INDIRECT, None, None))
        );
        assert_eq!(
            parse_register("- ( AX)").unwrap(),
            (Register(9 | IND_PRE_DEC, None, None))
        );
        assert_eq!(
            parse_register("CL +").unwrap(),
            (Register(5 | POST_INC, None, None))
        );
        assert_eq!(
            parse_register("( DX + 10)").unwrap(),
            (Register(12 | IND_OFFSET_NUM, None, Some(10)))
        );
        assert_eq!(
            parse_register("(DX + BH )").unwrap(),
            (Register(12 | IND_OFFSET_REG, Some(2), None))
        );
        assert_eq!(
            parse_register("( CX + AX)").unwrap(),
            (Register(11 | IND_OFFSET_EXT_REG, Some(9), None))
        );
    }

    #[test]
    fn test_register_detection() {
        assert_eq!(detect_register("ah", "ah").unwrap(), 0);
        assert_eq!(detect_register("al", "al").unwrap(), 1);
        assert_eq!(detect_register("bh", "bh").unwrap(), 2);
        assert_eq!(detect_register("bl", "bl").unwrap(), 3);
        assert_eq!(detect_register("ch", "ch").unwrap(), 4);
        assert_eq!(detect_register("cl", "cl").unwrap(), 5);
        assert_eq!(detect_register("dh", "dh").unwrap(), 6);
        assert_eq!(detect_register("dl", "dl").unwrap(), 7);
        assert_eq!(detect_register("flg", "flg").unwrap(), 8);
        assert_eq!(detect_register("ax", "ax").unwrap(), 9);
        assert_eq!(detect_register("bx", "bx").unwrap(), 10);
        assert_eq!(detect_register("cx", "cx").unwrap(), 11);
        assert_eq!(detect_register("dx", "dx").unwrap(), 12);

        assert!(detect_register("", "").is_err());
        assert!(detect_register("", "a").is_err());
        assert!(detect_register("", "al)").is_err());
        assert!(detect_register("", "h").is_err());
        assert!(detect_register("", "x").is_err());
        assert!(detect_register("", "yh").is_err());
    }

    #[test]
    fn test_indirect_detection() {
        assert_eq!(detect_indirect("(al)", "(al)").unwrap(), (true, "al"));
        assert_eq!(detect_indirect("(ax)", "(ax)").unwrap(), (true, "ax"));
        assert_eq!(
            detect_indirect("(al+ax)", "(al+ax)").unwrap(),
            (true, "al+ax")
        );
        assert_eq!(
            detect_indirect("(ax+500)", "(ax+500)").unwrap(),
            (true, "ax+500")
        );

        assert_eq!(detect_indirect("al)", "al)").unwrap(), (false, "al)"));

        assert!(detect_indirect("(", "(").is_err());
        assert!(detect_indirect("(al+500", "(al+500").is_err());
    }

    #[test]
    fn test_ppid_detection() {
        assert_eq!(detect_ppid("-al"), (Some((PP::Pre, ID::Dec)), "al"));
        assert_eq!(detect_ppid("+ax"), (Some((PP::Pre, ID::Inc)), "ax"));
        assert_eq!(detect_ppid("bx-"), (Some((PP::Post, ID::Dec)), "bx"));
        assert_eq!(detect_ppid("dh+"), (Some((PP::Post, ID::Inc)), "dh"));
        assert_eq!(detect_ppid("-(ax)"), (Some((PP::Pre, ID::Dec)), "(ax)"));
        assert_eq!(detect_ppid("+(bx)"), (Some((PP::Pre, ID::Inc)), "(bx)"));
        assert_eq!(detect_ppid("(cx)-"), (Some((PP::Post, ID::Dec)), "(cx)"));
        assert_eq!(detect_ppid("(cx)+"), (Some((PP::Post, ID::Inc)), "(cx)"));

        assert_eq!(detect_ppid("cx"), (None, "cx"));
        assert_eq!(detect_ppid("(bx)"), (None, "(bx)"));
        assert_eq!(detect_ppid("(ax+al)"), (None, "(ax+al)"));
    }

    #[test]
    fn test_offset_detection() {
        assert_eq!(
            detect_offset("", "100").unwrap(),
            Offset {
                num: Some(100),
                ..Offset::default()
            }
        );
        assert_eq!(
            detect_offset("", "x100").unwrap(),
            Offset {
                num: Some(256),
                ..Offset::default()
            }
        );
        assert_eq!(
            detect_offset("", "-124").unwrap(),
            Offset {
                num: Some(65412),
                ..Offset::default()
            }
        );
        assert_eq!(
            detect_offset("", "bl").unwrap(),
            Offset {
                reg: Some(3),
                ..Offset::default()
            }
        );
        assert_eq!(
            detect_offset("", "dx").unwrap(),
            Offset {
                ext_reg: Some(12),
                ..Offset::default()
            }
        );

        assert!(detect_offset("", "(ax)").is_err());
        assert!(detect_offset("", "90000").is_err());
        assert!(detect_offset("", "xFFFFF").is_err());
        assert!(detect_offset("", "-ax").is_err());
        assert!(detect_offset("", "al+").is_err());
    }

    #[test]
    fn test_num_detection() {
        assert_eq!(detect_num("", "0").unwrap().unwrap(), 0);
        assert_eq!(detect_num("", "x0").unwrap().unwrap(), 0);
        assert_eq!(detect_num("", "b0").unwrap().unwrap(), 0);
        assert_eq!(detect_num("", "1").unwrap().unwrap(), 1);
        assert_eq!(detect_num("", "-1").unwrap().unwrap(), 65535);
        assert_eq!(detect_num("", "x1").unwrap().unwrap(), 1);
        assert_eq!(detect_num("", "b1").unwrap().unwrap(), 1);
        assert_eq!(detect_num("", "'A'").unwrap().unwrap(), 65);
        assert_eq!(detect_num("", "'\''").unwrap().unwrap(), 39);
    }
}