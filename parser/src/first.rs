use super::syntax::{
    NodeKind, NodeKindSet, TokenKind, TokenKindSet, INFIX_OPS, LITERALS, PREFIX_OPS, TRIVIA,
};
use enumset::enum_set;

use NodeKind::*;
use AliasKind::*;
use TokenKind::*;

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
    STMT,
    EXPR,
    LEVEL_INFIX,
    LEVEL_PREFIX,
    LEVEL_POSTFIX,
    LEVEL_ATOM,
}

pub(crate) type AliasKindSet = enumset::EnumSet<AliasKind>;

pub(crate) trait First {
    fn first(self) -> TokenKindSet;
}

impl TokenKind {
    pub(crate) fn starts(self, item: impl First) -> bool {
        self.is(item.first())
    }
}

impl First for TokenKind {
    fn first(self) -> TokenKindSet {
        TokenKindSet::only(self)
    }
}

impl First for TokenKindSet {
    fn first(self) -> TokenKindSet {
        self
    }
}

impl First for NodeKind {
    fn first(self) -> TokenKindSet {
        match self {
            PROGRAM => DEFN.first(),
            DEFN_FN => enum_set!(KW_FN),
            EXPR_BLOCK => enum_set!(LBRACE),
            STMT_ASSIGN => EXPR.first(),
            STMT_EXPR => EXPR.first(),
            STMT_IF => enum_set!(KW_IF),
            STMT_LET => enum_set!(KW_LET),
            EXPR_CLOSURE => PARAMS_CLOSURE.first(),
            EXPR_IF => enum_set!(KW_IF),
            EXPR_INFIX => LEVEL_PREFIX.first(),
            EXPR_PREFIX => OP_PREFIX.first(),
            EXPR_CALL => LEVEL_POSTFIX.first(),
            EXPR_SELECT => LEVEL_POSTFIX.first(),
            EXPR_VAR => enum_set!(IDENT),
            EXPR_LIT => LITERALS,
            EXPR_TUPLE => enum_set!(LPAREN),
            EXPR_PAREN => enum_set!(LPAREN),
            PARAMS_CLOSURE => enum_set!(BAR),
            PARAMS_FN => enum_set!(LPAREN),
            BINDER => enum_set!(KW_MUT | IDENT),
            ARGS => enum_set!(LPAREN),
            OP_INFIX => INFIX_OPS,
            OP_PREFIX => PREFIX_OPS,
            ERROR => !enum_set!(TRIVIA | EOF),
        }
    }
}

impl First for NodeKindSet {
    fn first(self) -> TokenKindSet {
        let mut set = TokenKindSet::empty();
        for node in self {
            set |= node.first();
        }
        set
    }
}

impl First for AliasKind {
    fn first(self) -> TokenKindSet {
        // self.expand().first()
        match self {
            DEFN => enum_set!(DEFN_FN).first(),
            STMT => enum_set!(STMT_LET | STMT_IF).first() | EXPR.first(),
            EXPR => enum_set!(EXPR_CLOSURE | EXPR_IF).first() | LEVEL_INFIX.first(),
            LEVEL_INFIX => LEVEL_PREFIX.first(),
            LEVEL_PREFIX => EXPR_PREFIX.first() | LEVEL_POSTFIX.first(),
            LEVEL_POSTFIX => LEVEL_ATOM.first(),
            LEVEL_ATOM => {
                enum_set!(EXPR_LIT | EXPR_VAR | EXPR_TUPLE | EXPR_PAREN | EXPR_BLOCK).first()
            }
        }
    }
}

impl First for AliasKindSet {
    fn first(self) -> TokenKindSet {
        let mut set = TokenKindSet::empty();
        for union in self {
            set |= union.first();
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_kind_first_terminates() {
        for token in TokenKindSet::all() {
            token.first();
        }
    }

    #[test]
    fn node_kind_first_terminates() {
        for node in NodeKindSet::all() {
            node.first();
        }
    }

    #[test]
    fn alias_kind_first_terminates() {
        for alias in AliasKindSet::all() {
            alias.first();
        }
    }
}
