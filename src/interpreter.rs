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

    let args = parts
        .collect::<Vec<&str>>()
        .join("")
        .split(',')
        .map(|str| str.to_string())
        .collect::<Vec<String>>();
    if let Some(command) = line.command.as_mut() {
        command.1 = args;
    }
    Ok(line)
}

#[cfg(test)]
mod test {
    use crate::interpret_line;

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
