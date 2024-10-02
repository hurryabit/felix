use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use felix_common::{Problem, Severity};
use felix_parser::{rules, Parser};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct ParseResult {
    problems: Vec<Problem>,
}

#[wasm_bindgen]
pub fn parse(input: &str) -> ParseResult {
    let mapper = felix_common::srcloc::Mapper::new(input);
    let parser = Parser::new(input);
    let result = parser.parse(rules::root);
    let problems = result
        .errors
        .into_iter()
        .map(|error| {
            let span = error.span;
            Problem {
                start: mapper.src_loc(span.start),
                end: mapper.src_loc(span.end),
                severity: Severity::Error,
                source: error.rule,
                message: format!("Found {:?}, expected {:?}.", error.found, error.expected),
            }
        })
        .collect();
    ParseResult { problems }
}
