use super::{node, token};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FelixLang {}

impl rowan::Language for FelixLang {
    type NodeKind = node::NodeKind;

    type TokenKind = token::TokenKind;

    fn node_kind_from_raw(raw: rowan::NodeKind) -> Self::NodeKind {
        raw.try_into().expect("invalid node kind")
    }

    fn node_kind_to_raw(kind: Self::NodeKind) -> rowan::NodeKind {
        kind.into()
    }

    fn token_kind_from_raw(raw: rowan::TokenKind) -> Self::TokenKind {
        raw.try_into().expect("invalid token kind")
    }

    fn token_kind_to_raw(kind: Self::TokenKind) -> rowan::TokenKind {
        kind.into()
    }
}
