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
