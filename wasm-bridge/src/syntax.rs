use serde::Serialize;
use tsify_next::Tsify;

use felix_common::{srcloc::Mapper, SrcLoc};
use felix_parser;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Node {
    id: String,
    start: SrcLoc,
    end: SrcLoc,
    kind: String,
    children: Vec<Element>,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Token {
    id: String,
    start: SrcLoc,
    end: SrcLoc,
    kind: String,
    text: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Tsify)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "tag")]
#[tsify(into_wasm_abi)]
pub enum Element {
    Node(Node),
    Token(Token),
}

impl Node {
    pub fn from_parser_node(
        node: felix_parser::SyntaxNode,
        id: String,
        include_trivia: bool,
        mapper: &Mapper,
    ) -> Node {
        let span = node.text_range();
        let children = node
            .children_with_tokens()
            .enumerate()
            .filter_map(|(index, element)| {
                if element.kind().is_trivia() && !include_trivia {
                    None
                } else {
                    Some(Element::from_parser_element(
                        element,
                        format!("{}.{}", id, index),
                        include_trivia,
                        mapper,
                    ))
                }
            })
            .collect();
        Node {
            id,
            start: mapper.src_loc(span.start().into()),
            end: mapper.src_loc(span.end().into()),
            kind: format!("{:?}", node.kind()),
            children,
        }
    }
}

impl Token {
    pub fn from_parser_token(
        token: felix_parser::SyntaxToken,
        id: String,
        mapper: &Mapper,
    ) -> Token {
        let span = token.text_range();
        Token {
            id,
            start: mapper.src_loc(span.start().into()),
            end: mapper.src_loc(span.end().into()),
            kind: format!("{:?}", token.kind()),
            text: token.text().to_string(),
        }
    }
}

impl Element {
    pub fn from_parser_element(
        element: felix_parser::SyntaxElement,
        id: String,
        include_trivia: bool,
        mapper: &Mapper,
    ) -> Element {
        match element {
            felix_parser::SyntaxElement::Node(node) => {
                Element::Node(Node::from_parser_node(node, id, include_trivia, mapper))
            }
            felix_parser::SyntaxElement::Token(token) => {
                Element::Token(Token::from_parser_token(token, id, mapper))
            }
        }
    }
}
