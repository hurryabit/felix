use enumset::enum_set;
use strum::VariantArray;

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialOrd, Ord, enumset::EnumSetType, strum::Display, VariantArray)]
#[repr(u16)]
#[enumset(repr = "u64")]
pub enum NodeKind {
    PROGRAM,

    EXPR_ABS,
    EXPR_APP,
    EXPR_LET,
    EXPR_PAREN,
    EXPR_VAR,
    EXPR_UNIT,
    EXPR_META, // Placeholder for expressions in generated syntax.

    BINDER,
    NAME,

    TYPE_ARROW,
    TYPE_PAREN,
    TYPE_VAR,
    TYPE_UNIT,
    TYPE_META, // Placeholder for types in generated syntax.

    ERROR,
}

pub type NodeKindSet = enumset::EnumSet<NodeKind>;

pub type Node = rowan::SyntaxNode<super::lang::FelixLang>;

impl NodeKind {
    pub const LAST: Self = Self::VARIANTS[Self::VARIANTS.len() - 1];

    pub const EXPR: NodeKindSet =
        enum_set!(Self::EXPR_ABS | Self::EXPR_APP | Self::EXPR_LET | Self::EXPR_ATOM | Self::EXPR_META);
    pub const EXPR_ATOM: NodeKindSet =
        enum_set!(Self::EXPR_PAREN | Self::EXPR_VAR | Self::EXPR_UNIT);

    pub const TYPE: NodeKindSet = enum_set!(Self::TYPE_ARROW | Self::TYPE_ATOM | Self::TYPE_META);
    pub const TYPE_ATOM: NodeKindSet =
        enum_set!(Self::TYPE_PAREN | Self::TYPE_VAR | Self::TYPE_UNIT);
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

    #[test]
    fn expr_set_correct() {
        for node in NodeKind::VARIANTS {
            assert_eq!(
                node.to_string().starts_with("EXPR"),
                NodeKind::EXPR.contains(*node)
            );
        }
    }

    #[test]
    fn type_set_correct() {
        for node in NodeKind::VARIANTS {
            assert_eq!(
                node.to_string().starts_with("TYPE"),
                NodeKind::TYPE.contains(*node)
            );
        }
    }
}
