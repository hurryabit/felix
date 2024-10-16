#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Hash,
    PartialOrd,
    Ord,
    enumset::EnumSetType,
    logos::Logos,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
    strum::FromRepr,
)]
#[repr(u16)]
#[enumset(repr = "u64")]
pub enum TokenKind {
    // Keywords
    #[token("and")]
    KW_AND,
    #[token("else")]
    KW_ELSE,
    #[token("false")]
    KW_FALSE,
    #[token("fn")]
    KW_FN,
    #[token("if")]
    KW_IF,
    #[token("let")]
    KW_LET,
    #[token("mut")]
    KW_MUT,
    #[token("rec")]
    KW_REC,
    #[token("true")]
    KW_TRUE,

    // Delimiters
    #[token("<")]
    LANGLE,
    #[token(">")]
    RANGLE,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,
    #[token("[")]
    LBRACKET,
    #[token("]")]
    RBRACKET,
    #[token(")")]
    RPAREN,
    #[token("(")]
    LPAREN,

    // Operator/separator
    #[token("&&")]
    AMPER_AMPER,
    #[token("!")]
    BANG,
    #[token("!=")]
    BANG_EQUALS,
    #[token("|")]
    BAR,
    #[token("||")]
    BAR_BAR,
    #[token(":")]
    COLON,
    #[token(",")]
    COMMA,
    #[token(".")]
    DOT,
    #[token("=")]
    EQUALS,
    #[token("==")]
    EQUALS_EQUALS,
    #[token("<=")]
    LANGLE_EQUALS,
    #[token("%")]
    PERCENT,
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("->")]
    MINUS_RANGLE,
    #[token("?")]
    QUERY,
    #[token(">=")]
    RANGLE_EQUALS,
    #[token(";")]
    SEMI,
    #[token("/")]
    SLASH,
    #[token("*")]
    STAR,

    // Regular expressions
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    IDENT,
    #[regex(r"0|[1-9][0-9]*")]
    LIT_NAT,
    #[regex(r"\s+")]
    WHITESPACE,
    #[regex(r"//[^\n]*\n")] // TODO(MH): Support /* */ comments.
    COMMENT,

    // Special
    UNKNOWN, // Unknown token, used for error recovery.
    EOF,     // End-of-file.
}
use TokenKind::*;

pub type TokenKindSet = enumset::EnumSet<TokenKind>;

pub type Token = rowan::SyntaxToken<super::lang::FelixLang>;

pub const INFIX_OPS: TokenKindSet = enumset::enum_set!(
    PLUS | MINUS
        | STAR
        | SLASH
        | PERCENT
        | EQUALS_EQUALS
        | BANG_EQUALS
        | LANGLE
        | LANGLE_EQUALS
        | RANGLE
        | RANGLE_EQUALS
        | AMPER_AMPER
        | BAR_BAR
);
pub const PREFIX_OPS: TokenKindSet = enumset::enum_set!(BANG);
pub const LITERALS: TokenKindSet = enumset::enum_set!(LIT_NAT | KW_FALSE | KW_TRUE);
pub const TRIVIA: TokenKindSet = enumset::enum_set!(WHITESPACE | COMMENT);

impl TokenKind {
    #[inline(always)]
    pub fn is(self, set: TokenKindSet) -> bool {
        set.contains(self)
    }

    #[inline(always)]
    pub fn is_trivia(self) -> bool {
        self.is(TRIVIA)
    }
}

impl TryFrom<rowan::TokenKind> for TokenKind {
    type Error = ();

    fn try_from(value: rowan::TokenKind) -> Result<Self, Self::Error> {
        Self::from_repr(value.0).ok_or(())
    }
}

impl From<TokenKind> for rowan::TokenKind {
    fn from(value: TokenKind) -> Self {
        Self(value as u16)
    }
}
