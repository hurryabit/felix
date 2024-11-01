use strum::VariantArray;

#[allow(non_camel_case_types)]
#[derive(
    Debug, Hash, PartialOrd, Ord, enumset::EnumSetType, logos::Logos, strum::Display, VariantArray,
)]
#[repr(u16)]
#[enumset(repr = "u64")]
pub enum TokenKind {
    // Keywords
    #[token("in")]
    KW_IN,
    #[token("let")]
    KW_LET,
    #[token("unit")]
    KW_UNIT,

    // Greek letters
    #[token("λ")]
    GR_LAMBDA_LOWER,

    // Builtin types
    #[token("Unit")]
    TY_UNIT,

    // Delimiters
    #[token(")")]
    RPAREN,
    #[token("(")]
    LPAREN,

    // Operators
    #[token("->")]
    OP_ARROW,

    // Separators
    #[token(":")]
    COLON,
    #[token(".")]
    DOT,
    #[token("=")]
    EQUALS,

    // Regular expressions
    #[regex(r"_*[a-z][A-Za-z0-9_]*")]
    ID_EXPR,
    #[regex(r"_*[A-Z][A-Za-z0-9_]*")]
    ID_TYPE,
    #[regex(r"\s+")]
    WHITESPACE,
    #[regex(r"#[^\n]*\n?")]
    COMMENT,

    // Special
    UNKNOWN, // Unknown token, used for error recovery.
    EOF,     // End-of-file.
}
use TokenKind::*;

pub type TokenKindSet = enumset::EnumSet<TokenKind>;

pub type Token = rowan::SyntaxToken<super::lang::FelixLang>;

// pub const BUILTIN_TYPES: TokenKindSet =
//     enumset::enum_set!(TY_BOOL | TY_BOT | TY_INT | TY_TOP | TY_UNIT);
// pub const TYPE_INFIX_OPS: TokenKindSet = enumset::enum_set!(ARROW | UNION | INTER | TIMES);
// pub const TYPE_PREFIX_OPS: TokenKindSet = enumset::enum_set!(COMPL);
// pub const EXPR_INFIX_OPS: TokenKindSet = enumset::enum_set!(
//     PLUS | MINUS | TIMES | DIV | MOD | EQ_EQ | NOT_EQ | LT | LT_EQ | GT | GT_EQ | AND | OR
// );
// pub const EXPR_PREFIX_OPS: TokenKindSet = enumset::enum_set!(NOT);
// pub const LITERALS: TokenKindSet = enumset::enum_set!(LIT_NAT | KW_FALSE | KW_TRUE);
pub const TRIVIA: TokenKindSet = enumset::enum_set!(WHITESPACE | COMMENT);

impl TokenKind {
    pub const LAST: Self = Self::VARIANTS[Self::VARIANTS.len() - 1];

    #[inline(always)]
    pub fn is(self, set: TokenKindSet) -> bool {
        set.contains(self)
    }

    #[inline(always)]
    pub fn is_trivia(self) -> bool {
        self.is(TRIVIA)
    }
}

impl From<TokenKind> for u16 {
    fn from(kind: TokenKind) -> Self {
        kind as u16
    }
}

impl TryFrom<u16> for TokenKind {
    type Error = u16;

    fn try_from(repr: u16) -> Result<Self, Self::Error> {
        if repr <= Self::LAST as u16 {
            Ok(unsafe { std::mem::transmute::<u16, Self>(repr) })
        } else {
            Err(repr)
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use strum::VariantArray;

    use super::*;

    #[test]
    fn repr_is_index() {
        for (index, node) in TokenKind::VARIANTS.iter().enumerate() {
            assert_eq!((*node as u16) as usize, index, "failed for {:?}", node);
        }
    }

    #[test]
    fn last_is_max() {
        for node in TokenKind::VARIANTS {
            assert!(*node <= TokenKind::LAST, "failed for {:?}", node);
        }
    }

    #[test]
    fn try_from_repr_roundtrip() {
        for token in TokenKind::VARIANTS {
            assert_eq!(TokenKind::try_from(*token as u16), Ok(*token));
        }
    }

    #[test]
    fn try_from_past_last_fails() {
        assert_matches!(TokenKind::try_from(TokenKind::LAST as u16 + 1), Err(_));
    }
}
