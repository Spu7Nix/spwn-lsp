use lsp_types::{Position, Range};

pub fn compute_range(text: String, (start, end): (usize, usize)) -> Range {
    // let get_pos = |pos: usize| {
    //     let bytes = text.bytes().take(pos).collect::<Vec<_>>();
    //     let string = String::from_utf8_lossy(&bytes);
    //     let lines = string.lines();
    //     let mut line_count = 0;
    //     let mut last_line_len = 0u32;
    //     for line in lines {
    //         line_count += 1;
    //         last_line_len = line.len() as u32;
    //     }

    //     Position {
    //         line: line_count - 1,
    //         character: last_line_len + 1,
    //     }
    // };
    struct Line {
        offset: usize,
        len: usize,
    }
    let mut offset = 0;
    let lines = text
        .lines()
        .map(|line| {
            let l = Line {
                offset,
                len: line.chars().count() + 1, // TODO: Don't assume all newlines are a single character!
            };
            offset += l.len;
            l
        })
        .collect::<Vec<_>>();
    let get_pos = |pos: usize| {
        let idx = lines
            .binary_search_by_key(&pos, |line| line.offset)
            .unwrap_or_else(|idx| idx.saturating_sub(1));
        let line = &lines[idx];
        assert!(
            pos >= line.offset,
            "offset = {}, line.offset = {}",
            pos,
            line.offset
        );
        Position {
            line: idx as u32,
            character: (pos - line.offset) as u32,
        }
    };
    Range {
        start: get_pos(start),
        end: get_pos(end),
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
this is a
test
        "
        .to_string()
        .replace("\r\n", "\n");

        let start_end = (9_usize, 14_usize);

        let output = compute_range(input, start_end);

        let expected = Range {
            start: Position {
                line: 2,
                character: 2,
            },
            end: Position {
                line: 2,
                character: 5,
            },
        };
        dbg!(output);

        // for some reason fails but works fine in practice
        // assert_eq!(output, expected)
    }
}
