use crate::syntax::{
    AliasKind, AliasKindSet, NodeKind, NodeKindSet, TokenKind, TokenKindSet, BUILTIN_TYPES,
    EXPR_INFIX_OPS, EXPR_PREFIX_OPS, LITERALS, TRIVIA, TYPE_INFIX_OPS, TYPE_PREFIX_OPS,
};
use enumset::enum_set;

use AliasKind::*;
use NodeKind::*;
use TokenKind::*;

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
            PROGRAM => DEFN.first() | EOF,
            DEFN_TYPE | DEFN_TYPE_REC => enum_set!(KW_TYPE),
            BIND_TYPE => enum_set!(IDENT),
            TYPE_INFIX => LEVEL_TYPE_INFIX.first(),
            TYPE_PREFIX => TYPE_PREFIX_OPS,
            TYPE_BUILTIN => BUILTIN_TYPES,
            TYPE_REF => enum_set!(IDENT),
            TYPE_PAREN => enum_set!(LPAREN),
            OP_TYPE_INFIX => TYPE_INFIX_OPS,
            OP_TYPE_PREFIX => TYPE_PREFIX_OPS,
            DEFN_LET | DEFN_LET_REC => enum_set!(KW_LET),
            BIND_EXPR => PAT.first(),
            PAT_IDENT => enum_set!(IDENT),
            PAT_UNIT | PAT_PAIR => enum_set!(LPAREN),
            EXPR_FUN => enum_set!(KW_FUN),
            EXPR_LET | EXPR_LET_REC => enum_set!(KW_LET),
            EXPR_IF => enum_set!(KW_IF),
            EXPR_INFIX => LEVEL_EXPR_INFIX.first(),
            EXPR_PREFIX => EXPR_PREFIX_OPS,
            EXPR_APP => LEVEL_EXPR_APP.first(),
            EXPR_LIT => LITERALS,
            EXPR_REF => enum_set!(IDENT),
            EXPR_UNIT | EXPR_PAIR => enum_set!(LPAREN),
            EXPR_PAREN => enum_set!(LPAREN),
            OP_EXPR_INFIX => EXPR_INFIX_OPS,
            OP_EXPR_PREFIX => EXPR_PREFIX_OPS,
            ERROR => TRIVIA | EOF,
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
            DEFN => (DEFN_TYPE | DEFN_TYPE_REC | DEFN_LET | DEFN_LET_REC).first(),
            TYPE => LEVEL_TYPE_INFIX.first(),
            LEVEL_TYPE_INFIX => LEVEL_TYPE_PREFIX.first(),
            LEVEL_TYPE_PREFIX => LEVEL_TYPE_ATOM.first() | TYPE_PREFIX_OPS,
            LEVEL_TYPE_ATOM => (TYPE_BUILTIN | TYPE_REF | TYPE_PAREN).first(),
            PAT => (PAT_IDENT | PAT_PAIR).first(),
            EXPR => (EXPR_FUN | EXPR_LET | EXPR_LET_REC | EXPR_IF).first() | LEVEL_EXPR_INFIX.first(),
            LEVEL_EXPR_INFIX => LEVEL_EXPR_PREFIX.first(),
            LEVEL_EXPR_PREFIX => LEVEL_EXPR_APP.first() | EXPR_PREFIX_OPS,
            LEVEL_EXPR_APP => LEVEL_EXPR_ATOM.first(),
            LEVEL_EXPR_ATOM => (EXPR_LIT | EXPR_REF | EXPR_UNIT | EXPR_PAIR | EXPR_PAREN).first(),
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
        fn can_start<'a>(
            token: TokenKind,
            rule: fn(&mut Parser<'a>, TokenKindSet) -> Result<()>,
        ) -> bool {
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
        let cases: Vec<(
            NodeOrAliasKind,
            fn(&mut Parser<'static>, TokenKindSet) -> Result<()>,
        )> = vec![
            (Alias(DEFN), Parser::defn),
            (Node(DEFN_TYPE), Parser::defn_type),
            (Node(DEFN_TYPE_REC), Parser::defn_type),
            (Node(BIND_TYPE), Parser::bind_type),
            (Alias(TYPE), Parser::type_),
            (Alias(LEVEL_TYPE_INFIX), Parser::level_type_infix),
            (Alias(LEVEL_TYPE_PREFIX), Parser::level_type_prefix),
            (Alias(LEVEL_TYPE_ATOM), Parser::level_type_atom),
            (Node(DEFN_LET), Parser::defn_let),
            (Node(DEFN_LET_REC), Parser::defn_let),
            (Node(BIND_EXPR), Parser::bind_expr),
            (Alias(PAT), Parser::pat),
            (Node(PAT_UNIT), Parser::pat_lparen),
            (Node(PAT_PAIR), Parser::pat_lparen),
            (Alias(EXPR), Parser::expr),
            (Node(EXPR_FUN), Parser::expr_fun),
            (Node(EXPR_LET), Parser::expr_let),
            (Node(EXPR_LET_REC), Parser::expr_let),
            (Node(EXPR_IF), Parser::expr_if),
            (Alias(LEVEL_EXPR_INFIX), Parser::level_expr_infix),
            (Alias(LEVEL_EXPR_PREFIX), Parser::level_expr_prefix),
            (Alias(LEVEL_EXPR_APP), Parser::level_expr_app),
            (Alias(LEVEL_EXPR_ATOM), Parser::level_expr_atom),
            (Node(EXPR_UNIT), Parser::expr_lparen),
            (Node(EXPR_PAIR), Parser::expr_lparen),
            (Node(EXPR_PAREN), Parser::expr_lparen),
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
