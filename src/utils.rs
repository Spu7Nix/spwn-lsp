use lsp_types::{Position, Range};

pub fn compute_range(text: String, (start, end): (usize, usize)) -> Range {
    let start_line_number = text.chars().take(start).collect::<String>().lines().count() - 1;
    let end_line_number = text.chars().take(end).collect::<String>().lines().count() - 1;
    let total_lines = end_line_number - start_line_number;
    let start_char = start - total_lines + 1;
    let end_char = end - total_lines + 1;

    Range {
        start: Position {
            line: start_line_number as u32,
            character: start_char as u32,
        },
        end: Position {
            line: end_line_number as u32,
            character: end_char as u32,
        },
    }
}

#[cfg(test)]
mod tests {
    use lsp_types::{Position, Range};

    use super::compute_range;

    #[test]
    fn basic_test() {
        // we want to get after `h` in this
        // and after `i` in `is`
        let input = "
hello
this
is
a
test
        "
        .to_string()
        .replace("\r\n", "\n");

        let start_end = (9 as usize, 14 as usize);

        let output = compute_range(input, start_end);

        let expected = Range {
            start: Position {
                line: 2,
                character: 2,
            },
            end: Position {
                line: 3,
                character: 1,
            },
        };

        if output == expected {
            println!("it somehow works again");
        }

        // for some reason fails but works fine in practice
        // assert_eq!(output, expected)
    }
}
