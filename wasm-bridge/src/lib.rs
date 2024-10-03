use serde::{Deserialize, Serialize};
use tsify_next::{declare, Tsify};
use wasm_bindgen::prelude::wasm_bindgen;

use felix_common::{Problem, Severity};
use felix_parser::{rules, Parser};

pub mod syntax;

#[declare]
pub type SyntaxNode = syntax::Node;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct ParseOptions {
    pub include_trivia: bool,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct ParseResult {
    pub problems: Vec<Problem>,
    pub syntax: syntax::Node,
}

#[wasm_bindgen]
pub fn parse(input: &str, options: ParseOptions) -> ParseResult {
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
    let syntax = syntax::Node::from_parser_node(
        result.syntax,
        String::from(""),
        options.include_trivia,
        &mapper,
    );
    ParseResult { problems, syntax }
}
