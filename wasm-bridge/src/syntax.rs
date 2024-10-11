use serde::Serialize;
use tsify_next::Tsify;

use felix_common::{srcloc::Mapper, SrcLoc};
use felix_parser::syntax as parser;

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
    pub fn fake() -> Self {
        Self {
            id: String::from(""),
            start: SrcLoc { line: 0, column: 0 },
            end: SrcLoc { line: 0, column: 0 },
            kind: String::from("FAKE"),
            children: vec![],
        }
    }

    pub fn from_parser_node(
        node: parser::Node,
        id: String,
        include_trivia: bool,
        mapper: &Mapper,
    ) -> Node {
        let span = node.text_range();
        let children = node
            .children_with_tokens()
            .enumerate()
            .filter_map(|(index, element)| {
                let id = format!("{}.{}", id, index);
                match element {
                    parser::Element::Node(node) => Some(Element::Node(Node::from_parser_node(
                        node,
                        id,
                        include_trivia,
                        mapper,
                    ))),
                    parser::Element::Token(token) => {
                        if include_trivia || !token.kind().is_trivia() {
                            Some(Element::Token(Token::from_parser_token(token, id, mapper)))
                        } else {
                            None
                        }
                    }
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
    pub fn from_parser_token(token: parser::Token, id: String, mapper: &Mapper) -> Token {
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
