use serde::{Deserialize, Serialize};
use tsify_next::{declare, Tsify};
use wasm_bindgen::prelude::wasm_bindgen;

use felix_common::Problem;
use felix_parser::Parser;

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
    pub syntax: syntax::Element,
}

#[wasm_bindgen]
pub fn parse(input: &str, options: ParseOptions) -> ParseResult {
    console_error_panic_hook::set_once();
    let mapper = felix_common::srcloc::Mapper::new(input);
    let parser = Parser::new(input, &mapper);
    let result = parser.run(Parser::program);
    let syntax = syntax::Element::Node(syntax::Node::from_parser_node(
        result.syntax,
        String::from(""),
        options.include_trivia,
        &mapper,
    ));
    ParseResult {
        problems: result.problems,
        syntax,
    }
}

#[wasm_bindgen]
pub fn type_system_name() -> String {
    felix_type_checker::stlc::get().name.clone()
}
