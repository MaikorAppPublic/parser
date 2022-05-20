use crate::ParseError::{EmptyLine, General};
use crate::{Line, ParseError};

pub fn interpret_line(line_num: usize, trimmed: &str) -> Result<Line, ParseError> {
    if trimmed.is_empty() {
        return Err(EmptyLine);
    }
    let mut parts = trimmed.split_whitespace();
    let mut line = Line::new(line_num, trimmed.to_string());
    match parts.next() {
        None => {
            return Err(General(
                line_num,
                trimmed.to_string(),
                String::from("not empty but no contents?"),
            ))
        }
        Some(part) => {
            if part.ends_with(':') {
                line.label = Some(part.to_string());
            } else {
                line.command = Some((part.to_string(), vec![]));
            }
        }
    }
    if line.command.is_none() {
        if let Some(part) = parts.next() {
            line.command = Some((part.to_string(), vec![]));
        }
    }

    let remaining = parts.collect::<Vec<&str>>().join("");
    if !remaining.is_empty() {
        let args = remaining
            .split(',')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        if let Some(command) = line.command.as_mut() {
            command.1 = args;
        }
    }
    Ok(line)
}

#[cfg(test)]
mod test {
    use crate::{interpret_line, Line};

    fn test_op(command: &str, input_args: &str, args: Vec<&str>) {
        let input = format!("{} {}", command, input_args);
        assert_eq!(
            interpret_line(0, &input).unwrap(),
            Line {
                num: 0,
                original: input,
                label: None,
                command: Some((
                    command.to_string(),
                    args.iter().map(|str| str.to_string()).collect()
                ))
            }
        );
    }

    #[test]
    fn test_interpreting() {
        test_op("Nop", "", vec![]);
        test_op("inc.w", "ax", vec!["ax"]);
        test_op("swap.b", "aH, Al", vec!["aH", "Al"]);
        test_op("swap.w", "ax, bx", vec!["ax", "bx"]);
        test_op("add.w", "( ax + al), 19", vec!["(ax+al)", "19"]);
        test_op("muls.b", "(ax)+, bh", vec!["(ax)+", "bh"]);
        test_op("cpy.b", "-(dl), $550", vec!["-(dl)", "$550"]);
        test_op("mcpy", "$5111, cx, 12", vec!["$5111", "cx", "12"]);
    }

    #[test]
    fn simple_test() {
        let line = interpret_line(10, "INC.B (AX)").unwrap();
        assert_eq!(line.num, 10);
        assert_eq!(line.original, "INC.B (AX)");
        assert_eq!(line.label, None);
        assert_eq!(
            line.command,
            Some((String::from("INC.B"), vec![String::from("(AX)")]))
        );
    }
}
