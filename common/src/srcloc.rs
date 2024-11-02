use serde::Serialize;
use tsify_next::Tsify;

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SrcSpan<L> {
    pub start: L,
    pub end: L,
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct SrcLoc {
    pub line: u32,
    pub column: u32,
}

impl SrcLoc {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Mapper between byte indices into the source buffer and source locations
/// in the form of line/column pairs.
#[derive(Debug, Eq, PartialEq)]
pub struct Mapper {
    line_starts: Vec<u32>,
}

impl Mapper {
    pub fn new(input: &str) -> Self {
        let input_len: u32 = input.len().try_into().expect("input too long");
        let mut line_starts: Vec<u32> = input
            .split_inclusive('\n')
            .scan(0, |index, line| {
                let res = *index;
                // NOTE(MH): This cast does not truncate because `input.len()`
                // fits into a u32.
                *index += line.len() as u32;
                Some(res)
            })
            .collect();
        if input.ends_with("\n") {
            line_starts.push(input_len);
        }
        Self { line_starts }
    }

    pub fn src_loc(&self, index: u32) -> SrcLoc {
        let line = self
            .line_starts
            .binary_search(&index)
            .unwrap_or_else(|x| x - 1);
        SrcLoc {
            // NOTE(MH): This cast does not truncate because the input's length
            // fits into a u32.
            line: line as u32,
            column: index - self.line_starts[line],
        }
    }
}

impl SrcSpan<u32> {
    pub fn from_range(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }

    pub fn into_range(self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_starts() {
        let cases = vec![
            ("", vec![]),
            ("a", vec![0]),
            ("a\n", vec![0, 2]),
            ("aa", vec![0]),
            ("a\nb", vec![0, 2]),
            ("a\nb\n", vec![0, 2, 4]),
            ("ab\ncd\n", vec![0, 3, 6]),
            ("\na", vec![0, 1]),
            ("a\r\nb\n\rc", vec![0, 3, 5]),
        ];
        for (input, expected_line_starts) in cases {
            let mapper = Mapper::new(input);
            let expected_line_starts: Vec<_> = expected_line_starts.into_iter().collect();
            assert_eq!(
                mapper.line_starts, expected_line_starts,
                "input: {:?}",
                input
            );
        }
    }

    #[test]
    fn test_translation() {
        let mapper = Mapper::new("ab\nc\nde\n\nf");
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
            (11, 4, 2),
            (100, 4, 91),
        ];
        for (index, line, column) in cases {
            assert_eq!(
                mapper.src_loc(index),
                SrcLoc { line, column },
                "index: {:?}",
                index
            );
        }
    }
}
