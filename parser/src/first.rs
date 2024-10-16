use super::syntax::{
    NodeKind, NodeKindSet, TokenKind, TokenKindSet, INFIX_OPS, LITERALS, PREFIX_OPS, TRIVIA,
};
use enumset::enum_set;

use AliasKind::*;
use NodeKind::*;
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
    BLOCK_INNER,
    EXPR,
    LEVEL_TERTIARY,
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
            BLOCK => enum_set!(LBRACE),
            STMT_ASSIGN => EXPR.first(),
            STMT_EXPR => EXPR.first(),
            STMT_IF => enum_set!(KW_IF),
            STMT_LET => enum_set!(KW_LET),
            EXPR_TERTIARY => LEVEL_INFIX.first(),
            EXPR_INFIX => LEVEL_PREFIX.first(),
            EXPR_PREFIX => OP_PREFIX.first(),
            EXPR_CALL => LEVEL_POSTFIX.first(),
            EXPR_SELECT => LEVEL_POSTFIX.first(),
            EXPR_VAR => enum_set!(IDENT),
            EXPR_LIT => LITERALS,
            EXPR_TUPLE => enum_set!(LPAREN),
            EXPR_FN => enum_set!(KW_FN),
            EXPR_PAREN => enum_set!(LPAREN),
            PARAMS => enum_set!(LPAREN),
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
            BLOCK_INNER => (STMT_LET | STMT_IF).first() | EXPR.first(),
            EXPR => LEVEL_TERTIARY.first(),
            LEVEL_TERTIARY => LEVEL_INFIX.first(),
            LEVEL_INFIX => LEVEL_PREFIX.first(),
            LEVEL_PREFIX => EXPR_PREFIX.first() | LEVEL_POSTFIX.first(),
            LEVEL_POSTFIX => LEVEL_ATOM.first(),
            LEVEL_ATOM => {
                (EXPR_LIT | EXPR_VAR | EXPR_TUPLE | EXPR_FN | EXPR_PAREN | BLOCK).first()
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
    use crate::{parser::Result, Parser};

    #[derive(Clone, Copy)]
    pub(crate) enum NodeOrAliasKind {
        Node(NodeKind),
        Alias(AliasKind),
    }
    use NodeOrAliasKind::*;

    impl First for NodeOrAliasKind {
        fn first(self) -> TokenKindSet {
            match self {
                Node(node) => node.first(),
                Alias(alias) => alias.first(),
            }
        }
    }

    impl std::fmt::Display for NodeOrAliasKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node(node) => node.fmt(f),
                Alias(alias) => alias.fmt(f),
            }
        }
    }

    fn compute_first<'a>(rule: fn(&mut Parser<'a>, TokenKindSet) -> Result<()>) -> TokenKindSet {
        fn can_start<'a>(token: TokenKind, rule: fn(&mut Parser<'a>, TokenKindSet) -> Result<()>) -> bool {
            if token.is(TRIVIA) {
                return false;
            }
            let mut parser = Parser::fake_from_tokens(vec![token, TokenKind::UNKNOWN]);
            let mut parser = parser.with_root(PROGRAM);
            match rule(&mut parser, TokenKindSet::empty()) {
                Ok(_) => true,
                Err(problem) => {
                    let start = problem.start.column;
                    assert!(start <= 1);
                    start == 1
                }
            }
        }

        TokenKindSet::all()
            .into_iter()
            .filter(|token| can_start(*token, rule))
            .collect::<TokenKindSet>()
    }

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

    #[test]
    fn node_first_matches() {
        let cases: Vec<(NodeOrAliasKind, fn(&mut Parser<'static>, TokenKindSet) -> Result<()>)> = vec![
            (Alias(DEFN), Parser::defn),
            (Node(DEFN_FN), Parser::defn_fn),
            (Node(BLOCK), Parser::block),
            (Alias(BLOCK_INNER), Parser::block_inner),
            (Node(STMT_IF), Parser::stmt_if),
            (Node(STMT_LET), Parser::stmt_let),
            (Alias(EXPR), Parser::expr),
            (Alias(LEVEL_TERTIARY), Parser::level_tertiary),
            (Alias(LEVEL_INFIX), Parser::level_infix),
            (Alias(LEVEL_PREFIX), Parser::level_prefix),
            (Alias(LEVEL_POSTFIX), Parser::level_postfix),
            (Alias(LEVEL_ATOM), Parser::level_atom),
            (Node(EXPR_FN), Parser::expr_fn),
            (Node(PARAMS), Parser::params),
            (Node(BINDER), Parser::binder),
            (Node(ARGS), Parser::args),
        ];

        for (node, rule) in cases {
            assert_eq!(
                compute_first(rule),
                node.first(),
                "implemented vs declared FIRST({})",
                node
            );
        }
    }
}
