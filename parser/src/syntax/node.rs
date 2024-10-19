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
pub enum NodeKind {
    PROGRAM,

    DEFN_TYPE,
    DEFN_TYPE_REC,
    BIND_TYPE,

    TYPE_INFIX,
    TYPE_PREFIX,
    TYPE_BUILTIN,
    TYPE_REF,
    TYPE_PAREN,

    OP_TYPE_INFIX,
    OP_TYPE_PREFIX,

    DEFN_LET,
    DEFN_LET_REC,
    BIND_EXPR,

    PAT_IDENT,
    PAT_UNIT,
    PAT_PAIR,

    EXPR_INFIX,
    EXPR_PREFIX,
    EXPR_APP,
    EXPR_LIT,
    EXPR_REF,
    EXPR_UNIT,
    EXPR_PAIR,
    EXPR_FUN,
    EXPR_LET,
    EXPR_LET_REC,
    EXPR_IF,
    EXPR_PAREN,

    OP_EXPR_INFIX,
    OP_EXPR_PREFIX,

    ERROR,
}

pub type NodeKindSet = enumset::EnumSet<NodeKind>;

pub type Node = rowan::SyntaxNode<super::lang::FelixLang>;

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
pub(crate) enum AliasKind {
    DEFN,
    TYPE,
    LEVEL_TYPE_INFIX,
    LEVEL_TYPE_PREFIX,
    LEVEL_TYPE_ATOM,
    PAT,
    EXPR,
    LEVEL_EXPR_INFIX,
    LEVEL_EXPR_PREFIX,
    LEVEL_EXPR_APP,
    LEVEL_EXPR_ATOM,
}

pub(crate) type AliasKindSet = enumset::EnumSet<AliasKind>;

impl TryFrom<rowan::NodeKind> for NodeKind {
    type Error = ();

    fn try_from(value: rowan::NodeKind) -> Result<Self, Self::Error> {
        Self::from_repr(value.0).ok_or(())
    }
}

impl From<NodeKind> for rowan::NodeKind {
    fn from(value: NodeKind) -> Self {
        Self(value as u16)
    }
}
