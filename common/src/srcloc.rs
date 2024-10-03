use std::fmt;

use serde::Serialize;
use tsify_next::Tsify;

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct SrcLoc {
    pub line: u32,
    pub column: u32,
}

/// Mapper between byte indices into the source buffer and source locations
/// in the form of line/column pairs.
#[derive(Debug, Eq, PartialEq)]
pub struct Mapper {
    line_starts: Vec<u32>,
}

impl Mapper {
    pub fn new(input: &str) -> Self {
        assert!(input.len() < u32::MAX as usize);
        let mut line_starts = Vec::new();
        let mut index = 0;
        line_starts.push(index);
        for line in input.lines() {
            // FIXME(MH): This assumes a newline character is just one byte,
            // which is not true on Windows.
            // NOTE(MH): The cast from usize to u32 is safe because of the
            // assert above.
            index += line.len() as u32 + 1;
            line_starts.push(index);
        }
        Self { line_starts }
    }

    pub fn src_loc(&self, loc: u32) -> SrcLoc {
        let line = self
            .line_starts
            .binary_search(&loc)
            .unwrap_or_else(|x| x - 1);
        SrcLoc {
            // NOTE(MH): This cast ist safe because of the assert in new.
            line: line as u32,
            column: loc - self.line_starts[line],
        }
    }
}

impl fmt::Display for SrcLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE(MH): Internally, positions are zero-based. The user gets to see
        // them one-based though.
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}

impl fmt::Debug for SrcLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_starts() {
        let cases = vec![
            ("", vec![0]),
            ("a", vec![0, 2]),
            ("a\n", vec![0, 2]),
            ("aa", vec![0, 3]),
            ("a\nb", vec![0, 2, 4]),
            ("a\nb\n", vec![0, 2, 4]),
            ("ab\ncd\n", vec![0, 3, 6]),
            ("\na", vec![0, 1, 3]),
        ];
        for (input, expected_line_starts) in cases {
            let humanizer = Mapper::new(input);
            let expected_line_starts: Vec<_> = expected_line_starts.into_iter().collect();
            assert_eq!(humanizer.line_starts, expected_line_starts);
        }
    }

    #[test]
    fn test_translation() {
        let humanizer = Mapper::new("ab\nc\nde\n\nf");
        let cases = vec![
            (0, 0, 0),
            (1, 0, 1),
            (2, 0, 2),
            (3, 1, 0),
            (4, 1, 1),
            (5, 2, 0),
            (6, 2, 1),
            (7, 2, 2),
            (8, 3, 0),
            (9, 4, 0),
            (10, 4, 1),
            (11, 5, 0),
            (100, 5, 89),
        ];
        for (loc, line, column) in cases {
            assert_eq!(humanizer.src_loc(loc), SrcLoc { line, column });
        }
    }
}
