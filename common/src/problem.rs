use serde::Serialize;
use tsify_next::Tsify;

use crate::SrcLoc;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[tsify(into_wasm_abi)]
pub enum Severity {
    Error,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Problem {
    pub start: SrcLoc,
    pub end: SrcLoc,
    pub severity: Severity,
    pub source: String,
    pub message: String,
}
