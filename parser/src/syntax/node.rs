use strum::VariantArray;

#[allow(non_camel_case_types)]
#[derive(
    Debug, Hash, PartialOrd, Ord, enumset::EnumSetType, strum::Display, VariantArray,
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

impl NodeKind {
    pub const LAST: Self = Self::VARIANTS[Self::VARIANTS.len() - 1];
}

impl From<NodeKind> for u16 {
    fn from(kind: NodeKind) -> Self {
        kind as u16
    }
}

impl TryFrom<u16> for NodeKind {
    type Error = u16;

    fn try_from(repr: u16) -> Result<Self, Self::Error> {
        if repr <= Self::LAST as u16 {
            Ok(unsafe { std::mem::transmute(repr) })
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
        for (index, node) in NodeKind::VARIANTS.iter().enumerate() {
            assert_eq!((*node as u16) as usize, index, "failed for {:?}", node);
        }
    }

    #[test]
    fn last_is_max() {
        for node in NodeKind::VARIANTS {
            assert!(*node <= NodeKind::LAST, "failed for {:?}", node);
        }
    }

    #[test]
    fn try_from_repr_rountrip() {
        for node in NodeKind::VARIANTS {
            assert_eq!(NodeKind::try_from(*node as u16), Ok(*node));
        }
    }

    #[test]
    fn try_from_past_last_fails() {
        assert_matches!(NodeKind::try_from(NodeKind::LAST as u16 + 1), Err(_));
    }
}
