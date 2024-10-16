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

    DEFN_FN,

    BLOCK,

    STMT_ASSIGN,
    STMT_EXPR,
    STMT_IF,
    STMT_LET,

    EXPR_TERTIARY,
    EXPR_INFIX,
    EXPR_PREFIX,
    EXPR_CALL,
    EXPR_SELECT,
    EXPR_VAR,
    EXPR_LIT,
    EXPR_TUPLE,
    EXPR_FN,
    EXPR_PAREN,

    PARAMS,
    BINDER,
    ARGS,

    OP_INFIX,
    OP_PREFIX,

    ERROR,
}

pub type NodeKindSet = enumset::EnumSet<NodeKind>;

pub type Node = rowan::SyntaxNode<super::lang::FelixLang>;

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
