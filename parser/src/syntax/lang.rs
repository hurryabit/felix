use std::fmt::Debug;

use rowan::Language;

use super::{node, token};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyntaxKind {
    Node(node::NodeKind),
    Token(token::TokenKind),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FelixLang {}

impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> Self {
        match kind {
            SyntaxKind::Node(node_kind) => node_kind as u16,
            SyntaxKind::Token(token_kind) => token_kind as u16 | 0x0100,
        }
    }
}

impl TryFrom<u16> for SyntaxKind {
    type Error = u16;

    fn try_from(repr: u16) -> Result<Self, Self::Error> {
        if repr & 0x0100 == 0 {
            node::NodeKind::try_from(repr).map(SyntaxKind::Node)
        } else {
            token::TokenKind::try_from(repr & !0x0100).map(SyntaxKind::Token)
        }
    }
}

impl rowan::Language for FelixLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        raw.0.try_into().expect("rowan::SyntaxKind should be valid")
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        match self {
            SyntaxKind::Node(_) => false,
            SyntaxKind::Token(token_kind) => token_kind.is_trivia(),
        }
    }
}

impl Debug for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node(node) => node.fmt(f),
            Self::Token(token) => token.fmt(f),
        }
    }
}

impl From<node::NodeKind> for rowan::SyntaxKind {
    fn from(kind: node::NodeKind) -> Self {
        FelixLang::kind_to_raw(SyntaxKind::Node(kind))
    }
}

impl From<token::TokenKind> for rowan::SyntaxKind {
    fn from(kind: token::TokenKind) -> Self {
        FelixLang::kind_to_raw(SyntaxKind::Token(kind))
    }
}

#[cfg(test)]
mod tests {
    use strum::VariantArray;

    use super::*;

    #[test]
    fn syntax_kind_u16_roundtrip() {
        for node_kind in node::NodeKind::VARIANTS {
            let kind = SyntaxKind::Node(*node_kind);
            let raw: u16 = kind.into();
            assert_eq!(SyntaxKind::try_from(raw), Ok(kind));
        }
        for token_kind in token::TokenKind::VARIANTS {
            let kind = SyntaxKind::Token(*token_kind);
            let raw: u16 = kind.into();
            assert_eq!(SyntaxKind::try_from(raw), Ok(kind));
        }
    }
}
