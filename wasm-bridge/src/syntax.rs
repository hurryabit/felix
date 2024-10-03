use serde::Serialize;
use tsify::Tsify;

use felix_common::{srcloc::Mapper, SrcLoc};
use felix_parser;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Node {
    start: SrcLoc,
    end: SrcLoc,
    kind: String,
    children: Vec<Element>,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Token {
    start: SrcLoc,
    end: SrcLoc,
    kind: String,
    text: String,
    is_trivia: bool,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "tag")]
#[tsify(into_wasm_abi)]
pub enum Element {
    Node(Node),
    Token(Token),
}

impl Node {
    pub fn from_parser_node(node: felix_parser::SyntaxNode, mapper: &Mapper) -> Node {
        let span = node.text_range();
        Node {
            start: mapper.src_loc(span.start().into()),
            end: mapper.src_loc(span.end().into()),
            kind: format!("{:?}", node.kind()),
            children: node
                .children_with_tokens()
                .map(|element| Element::from_parser_element(element, mapper))
                .collect(),
        }
    }
}

impl Token {
    pub fn from_parser_token(token: felix_parser::SyntaxToken, mapper: &Mapper) -> Token {
        let span = token.text_range();
        let kind = token.kind();
        Token {
            start: mapper.src_loc(span.start().into()),
            end: mapper.src_loc(span.end().into()),
            kind: format!("{:?}", kind),
            text: token.text().to_string(),
            is_trivia: kind.is_trivia(),
        }
    }
}

impl Element {
    pub fn from_parser_element(element: felix_parser::SyntaxElement, mapper: &Mapper) -> Element {
        match element {
            felix_parser::SyntaxElement::Node(node) => {
                Element::Node(Node::from_parser_node(node, mapper))
            }
            felix_parser::SyntaxElement::Token(token) => {
                Element::Token(Token::from_parser_token(token, mapper))
            }
        }
    }
}
