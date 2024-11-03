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
pub struct Mapper<'a> {
    input: &'a str,
    line_starts: Vec<u32>,
}

impl<'a> Mapper<'a> {
    pub fn new(input: &'a str) -> Self {
        if input.len() == 0 {
            // NOTE(MH): Handling the empty input special removes an edge case
            // from `Self::src_loc`.
            return Self {
                input,
                line_starts: vec![0],
            };
        }
        // NOTE(MH): Because we ensure that `input.len()` fits into a u32, all
        // casts into u32 below do not truncate.
        let input_len: u32 = input.len().try_into().expect("input too long");
        let mut line_starts: Vec<u32> = input
            .split_inclusive('\n')
            .scan(0, |index, line| {
                let res = *index;
                *index += line.len() as u32;
                Some(res)
            })
            .collect();
        if input.ends_with("\n") {
            line_starts.push(input_len);
        }
        Self { input, line_starts }
    }

    pub fn src_loc(&self, index: u32) -> SrcLoc {
        let line = self
            .line_starts
            .binary_search(&index)
            .unwrap_or_else(|x| x - 1);
        let index = index as usize;
        let line_start = self.line_starts[line] as usize;
        let line_index = index - line_start;
        let line_text = &self.input[line_start..];
        let column = line_text
            .char_indices()
            .take_while(|(i, _)| *i < line_index)
            .count();
        SrcLoc {
            line: line as u32,
            column: column as u32,
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
            ("", vec![0]),
            ("a", vec![0]),
            ("a\n", vec![0, 2]),
            ("aa", vec![0]),
            ("a\nb", vec![0, 2]),
            ("a\nb\n", vec![0, 2, 4]),
            ("ab\ncd\n", vec![0, 3, 6]),
            ("\na", vec![0, 1]),
            ("a\r\nb\n\rc", vec![0, 3, 5]),
            ("λ\nμ∀\n", vec![0, 3, 9]),
        ];
        for (input, expected) in cases {
            let mapper = Mapper::new(input);
            assert_eq!(mapper.line_starts, expected, "input: {:?}", input);
        }
    }

    #[test]
    fn test_translation() {
        let cases = vec![
            ("", SrcLoc::new(0, 0), vec![]),
            ("a", SrcLoc::new(0, 1), vec![]),
            ("\n", SrcLoc::new(1, 0), vec![]),
            ("\r\n", SrcLoc::new(1, 0), vec![(1, SrcLoc::new(0, 1))]),
            ("λ", SrcLoc::new(0, 1), vec![(1, SrcLoc::new(0, 1))]),
        ];
        for (input, end, indices) in cases {
            let mapper = Mapper::new(input);
            let start = SrcLoc::new(0, 0);
            for (index, src_loc) in [(0, start), (input.len() as u32, end), (u32::MAX, end)]
                .into_iter()
                .chain(indices)
            {
                assert_eq!(
                    mapper.src_loc(index),
                    src_loc,
                    "input: {}, index: {}",
                    input,
                    index
                );
            }
        }
    }

    #[test]
    fn test_translation_long() {
        let mapper = Mapper::new("ab\nc\nde\n\nfλgμ\n∀\nh\r\n");
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
            (12, 4, 2),
            (13, 4, 3),
            (14, 4, 4),
            (15, 4, 4),
            (16, 5, 0),
            (17, 5, 1),
            (18, 5, 1),
            (19, 5, 1),
            (20, 6, 0),
            (21, 6, 1),
            (22, 6, 2),
            (23, 7, 0),
            (24, 7, 0),
            (25, 7, 0),
            (99, 7, 0),
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
