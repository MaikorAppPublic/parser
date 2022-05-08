use crate::ParserError;
use crate::ParserError::{InvalidRegister, NotExtRegister};
use maikor_language::op_params::RegisterPPID;
use maikor_language::op_params::ID::*;
use maikor_language::op_params::PP::*;
use maikor_language::registers::{id, offset};

pub fn parse_register(value: &str) -> Result<u8, ParserError> {
    let mut register = value.to_string();
    let mut args = RegisterPPID::default();
    if register.starts_with('-') {
        args.ppid = Some((Pre, Dec));
        register = register.chars().skip(1).collect();
    } else if register.starts_with('+') {
        if args.ppid.is_some() {
            return Err(InvalidRegister(value.to_string()));
        }
        args.ppid = Some((Pre, Inc));
        register = register.chars().skip(1).collect();
    } else if register.ends_with('-') {
        if args.ppid.is_some() {
            return Err(InvalidRegister(value.to_string()));
        }
        args.ppid = Some((Post, Dec));
        register = register.chars().rev().skip(1).collect();
        register = register.chars().rev().collect();
    } else if register.ends_with('+') {
        if args.ppid.is_some() {
            return Err(InvalidRegister(value.to_string()));
        }
        args.ppid = Some((Post, Inc));
        register = register.chars().rev().skip(1).collect();
        register = register.chars().rev().collect();
    }
    if register.starts_with('(') && register.ends_with(')') {
        args.is_indirect = true;
        register = register.chars().rev().skip(1).collect();
        register = register.chars().rev().skip(1).collect();
    }
    let reg_id = id::from_name(&register)?;
    if args.is_indirect && id::size(reg_id) != 2 {
        return Err(NotExtRegister(value.to_string()));
    }
    let register = offset::from_id(reg_id as u8)?;
    let ppid_byte: u8 = args.into();
    Ok(register | ppid_byte)
}

#[cfg(test)]
mod test {
    use crate::registers::parse_register;
    use maikor_language::op_params::{INDIRECT, POST_DEC, POST_INC, PRE_DEC, PRE_INC};
    use maikor_language::registers::offset;

    #[test]
    fn check_direct() {
        assert_eq!(parse_register("AH").unwrap(), offset::AH as u8);
        assert_eq!(parse_register("AL").unwrap(), offset::AL as u8);
        assert_eq!(parse_register("BH").unwrap(), offset::BH as u8);
        assert_eq!(parse_register("BL").unwrap(), offset::BL as u8);
        assert_eq!(parse_register("CH").unwrap(), offset::CH as u8);
        assert_eq!(parse_register("CL").unwrap(), offset::CL as u8);
        assert_eq!(parse_register("DH").unwrap(), offset::DH as u8);
        assert_eq!(parse_register("DL").unwrap(), offset::DL as u8);
        assert_eq!(parse_register("AX").unwrap(), offset::AX as u8);
        assert_eq!(parse_register("BX").unwrap(), offset::BX as u8);
        assert_eq!(parse_register("CX").unwrap(), offset::CX as u8);
        assert_eq!(parse_register("DX").unwrap(), offset::DX as u8);
        assert_eq!(parse_register("FLG").unwrap(), offset::FLAGS as u8);
    }

    #[test]
    fn check_indirect() {
        assert_eq!(parse_register("(AX)").unwrap(), offset::AX as u8 | INDIRECT);
        assert_eq!(parse_register("(BX)").unwrap(), offset::BX as u8 | INDIRECT);
        assert_eq!(parse_register("(CX)").unwrap(), offset::CX as u8 | INDIRECT);
        assert_eq!(parse_register("(DX)").unwrap(), offset::DX as u8 | INDIRECT);
    }

    #[test]
    fn check_direct_pre_inc() {
        assert_eq!(parse_register("+AH").unwrap(), offset::AH as u8 | PRE_INC);
        assert_eq!(parse_register("+AL").unwrap(), offset::AL as u8 | PRE_INC);
        assert_eq!(parse_register("+BH").unwrap(), offset::BH as u8 | PRE_INC);
        assert_eq!(parse_register("+BL").unwrap(), offset::BL as u8 | PRE_INC);
        assert_eq!(parse_register("+CH").unwrap(), offset::CH as u8 | PRE_INC);
        assert_eq!(parse_register("+CL").unwrap(), offset::CL as u8 | PRE_INC);
        assert_eq!(parse_register("+DH").unwrap(), offset::DH as u8 | PRE_INC);
        assert_eq!(parse_register("+DL").unwrap(), offset::DL as u8 | PRE_INC);
        assert_eq!(parse_register("+AX").unwrap(), offset::AX as u8 | PRE_INC);
        assert_eq!(parse_register("+BX").unwrap(), offset::BX as u8 | PRE_INC);
        assert_eq!(parse_register("+CX").unwrap(), offset::CX as u8 | PRE_INC);
        assert_eq!(parse_register("+DX").unwrap(), offset::DX as u8 | PRE_INC);
        assert_eq!(
            parse_register("+FLG").unwrap(),
            offset::FLAGS as u8 | PRE_INC
        );
    }

    #[test]
    fn check_direct_pre_dec() {
        assert_eq!(parse_register("-AH").unwrap(), offset::AH as u8 | PRE_DEC);
        assert_eq!(parse_register("-AL").unwrap(), offset::AL as u8 | PRE_DEC);
        assert_eq!(parse_register("-BH").unwrap(), offset::BH as u8 | PRE_DEC);
        assert_eq!(parse_register("-BL").unwrap(), offset::BL as u8 | PRE_DEC);
        assert_eq!(parse_register("-CH").unwrap(), offset::CH as u8 | PRE_DEC);
        assert_eq!(parse_register("-CL").unwrap(), offset::CL as u8 | PRE_DEC);
        assert_eq!(parse_register("-DH").unwrap(), offset::DH as u8 | PRE_DEC);
        assert_eq!(parse_register("-DL").unwrap(), offset::DL as u8 | PRE_DEC);
        assert_eq!(parse_register("-AX").unwrap(), offset::AX as u8 | PRE_DEC);
        assert_eq!(parse_register("-BX").unwrap(), offset::BX as u8 | PRE_DEC);
        assert_eq!(parse_register("-CX").unwrap(), offset::CX as u8 | PRE_DEC);
        assert_eq!(parse_register("-DX").unwrap(), offset::DX as u8 | PRE_DEC);
        assert_eq!(
            parse_register("-FLG").unwrap(),
            offset::FLAGS as u8 | PRE_DEC
        );
    }

    #[test]
    fn check_direct_post_inc() {
        assert_eq!(parse_register("AH+").unwrap(), offset::AH as u8 | POST_INC);
        assert_eq!(parse_register("AL+").unwrap(), offset::AL as u8 | POST_INC);
        assert_eq!(parse_register("BH+").unwrap(), offset::BH as u8 | POST_INC);
        assert_eq!(parse_register("BL+").unwrap(), offset::BL as u8 | POST_INC);
        assert_eq!(parse_register("CH+").unwrap(), offset::CH as u8 | POST_INC);
        assert_eq!(parse_register("CL+").unwrap(), offset::CL as u8 | POST_INC);
        assert_eq!(parse_register("DH+").unwrap(), offset::DH as u8 | POST_INC);
        assert_eq!(parse_register("DL+").unwrap(), offset::DL as u8 | POST_INC);
        assert_eq!(parse_register("AX+").unwrap(), offset::AX as u8 | POST_INC);
        assert_eq!(parse_register("BX+").unwrap(), offset::BX as u8 | POST_INC);
        assert_eq!(parse_register("CX+").unwrap(), offset::CX as u8 | POST_INC);
        assert_eq!(parse_register("DX+").unwrap(), offset::DX as u8 | POST_INC);
        assert_eq!(
            parse_register("FLG+").unwrap(),
            offset::FLAGS as u8 | POST_INC
        );
    }

    #[test]
    fn check_direct_post_dec() {
        assert_eq!(parse_register("AH-").unwrap(), offset::AH as u8 | POST_DEC);
        assert_eq!(parse_register("AL-").unwrap(), offset::AL as u8 | POST_DEC);
        assert_eq!(parse_register("BH-").unwrap(), offset::BH as u8 | POST_DEC);
        assert_eq!(parse_register("BL-").unwrap(), offset::BL as u8 | POST_DEC);
        assert_eq!(parse_register("CH-").unwrap(), offset::CH as u8 | POST_DEC);
        assert_eq!(parse_register("CL-").unwrap(), offset::CL as u8 | POST_DEC);
        assert_eq!(parse_register("DH-").unwrap(), offset::DH as u8 | POST_DEC);
        assert_eq!(parse_register("DL-").unwrap(), offset::DL as u8 | POST_DEC);
        assert_eq!(parse_register("AX-").unwrap(), offset::AX as u8 | POST_DEC);
        assert_eq!(parse_register("BX-").unwrap(), offset::BX as u8 | POST_DEC);
        assert_eq!(parse_register("CX-").unwrap(), offset::CX as u8 | POST_DEC);
        assert_eq!(parse_register("DX-").unwrap(), offset::DX as u8 | POST_DEC);
        assert_eq!(
            parse_register("FLG-").unwrap(),
            offset::FLAGS as u8 | POST_DEC
        );
    }

    #[test]
    fn check_indirect_pre_inc() {
        assert_eq!(
            parse_register("-(AX)").unwrap(),
            offset::AX as u8 | INDIRECT | PRE_DEC
        );
        assert_eq!(
            parse_register("-(BX)").unwrap(),
            offset::BX as u8 | INDIRECT | PRE_DEC
        );
        assert_eq!(
            parse_register("-(CX)").unwrap(),
            offset::CX as u8 | INDIRECT | PRE_DEC
        );
        assert_eq!(
            parse_register("-(DX)").unwrap(),
            offset::DX as u8 | INDIRECT | PRE_DEC
        );
    }

    #[test]
    fn check_indirect_post_inc() {
        assert_eq!(
            parse_register("(AX)+").unwrap(),
            offset::AX as u8 | INDIRECT | POST_INC
        );
        assert_eq!(
            parse_register("(BX)+").unwrap(),
            offset::BX as u8 | INDIRECT | POST_INC
        );
        assert_eq!(
            parse_register("(CX)+").unwrap(),
            offset::CX as u8 | INDIRECT | POST_INC
        );
        assert_eq!(
            parse_register("(DX)+").unwrap(),
            offset::DX as u8 | INDIRECT | POST_INC
        );
    }

    #[test]
    fn check_indirect_pre_dec() {
        assert_eq!(
            parse_register("-(AX)").unwrap(),
            offset::AX as u8 | INDIRECT | PRE_DEC
        );
        assert_eq!(
            parse_register("-(BX)").unwrap(),
            offset::BX as u8 | INDIRECT | PRE_DEC
        );
        assert_eq!(
            parse_register("-(CX)").unwrap(),
            offset::CX as u8 | INDIRECT | PRE_DEC
        );
        assert_eq!(
            parse_register("-(DX)").unwrap(),
            offset::DX as u8 | INDIRECT | PRE_DEC
        );
    }

    #[test]
    fn check_indirect_post_dec() {
        assert_eq!(
            parse_register("(AX)-").unwrap(),
            offset::AX as u8 | INDIRECT | POST_DEC
        );
        assert_eq!(
            parse_register("(BX)-").unwrap(),
            offset::BX as u8 | INDIRECT | POST_DEC
        );
        assert_eq!(
            parse_register("(CX)-").unwrap(),
            offset::CX as u8 | INDIRECT | POST_DEC
        );
        assert_eq!(
            parse_register("(DX)-").unwrap(),
            offset::DX as u8 | INDIRECT | POST_DEC
        );
    }

    #[test]
    fn check_not_ext_reg() {
        assert!(parse_register("(AL)").is_err());
        assert!(parse_register("(AH)").is_err());
        assert!(parse_register("(BL)").is_err());
        assert!(parse_register("(BH)").is_err());
        assert!(parse_register("(CL)").is_err());
        assert!(parse_register("(CH)").is_err());
        assert!(parse_register("(DL)").is_err());
        assert!(parse_register("(DH)").is_err());
        assert!(parse_register("(FLG)").is_err());

        assert!(parse_register("-(AL)").is_err());
        assert!(parse_register("-(AH)").is_err());
        assert!(parse_register("-(BL)").is_err());
        assert!(parse_register("-(BH)").is_err());
        assert!(parse_register("-(CL)").is_err());
        assert!(parse_register("-(CH)").is_err());
        assert!(parse_register("-(DL)").is_err());
        assert!(parse_register("-(DH)").is_err());
        assert!(parse_register("-(FLG)").is_err());

        assert!(parse_register("+(AL)").is_err());
        assert!(parse_register("+(AH)").is_err());
        assert!(parse_register("+(BL)").is_err());
        assert!(parse_register("+(BH)").is_err());
        assert!(parse_register("+(CL)").is_err());
        assert!(parse_register("+(CH)").is_err());
        assert!(parse_register("+(DL)").is_err());
        assert!(parse_register("+(DH)").is_err());
        assert!(parse_register("+(FLG)").is_err());

        assert!(parse_register("(AL)-").is_err());
        assert!(parse_register("(AH)-").is_err());
        assert!(parse_register("(BL)-").is_err());
        assert!(parse_register("(BH)-").is_err());
        assert!(parse_register("(CL)-").is_err());
        assert!(parse_register("(CH)-").is_err());
        assert!(parse_register("(DL)-").is_err());
        assert!(parse_register("(DH)-").is_err());
        assert!(parse_register("(FLG)-").is_err());

        assert!(parse_register("(AL)+").is_err());
        assert!(parse_register("(AH)+").is_err());
        assert!(parse_register("(BL)+").is_err());
        assert!(parse_register("(BH)+").is_err());
        assert!(parse_register("(CL)+").is_err());
        assert!(parse_register("(CH)+").is_err());
        assert!(parse_register("(DL)+").is_err());
        assert!(parse_register("(DH)+").is_err());
        assert!(parse_register("(FLG)+").is_err());
    }

    #[test]
    fn check_invalid() {
        assert!(parse_register("G").is_err());
        assert!(parse_register("-AL+").is_err());
        assert!(parse_register("-(AL)+").is_err());
    }
}
