use serde::Serialize;
use tsify_next::Tsify;

use crate::{srcloc::Mapper, SrcLoc};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub enum Severity {
    ERROR,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Problem {
    pub start: SrcLoc,
    pub end: SrcLoc,
    pub severity: Severity,
    pub source: String,
    pub message: String,
}

impl<'a> Mapper<'a> {
    pub fn problem(
        &self,
        start: u32,
        end: u32,
        severity: Severity,
        source: String,
        message: String,
    ) -> Problem {
        let start = self.src_loc(start);
        let end = self.src_loc(end);
        Problem {
            start,
            end,
            severity,
            source,
            message,
        }
    }

    pub fn error(&self, start: u32, end: u32, source: String, message: String) -> Problem {
        self.problem(start, end, Severity::ERROR, source, message)
    }
}
