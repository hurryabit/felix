use crate::syntax::{NodeKind, NodeKindSet, TokenKind, TokenKindSet};

use NodeKind::*;

pub(crate) trait First {
    fn first(self) -> TokenKindSet;
}

impl TokenKind {
    pub(crate) fn starts(self, item: impl First) -> bool {
        self.is(item.first())
    }
}

impl First for NodeKind {
    fn first(self) -> TokenKindSet {
        match self {
            PROGRAM => NodeKind::EXPR.first(),
            EXPR_ABS => TokenKind::GR_LAMBDA_LOWER.into(),
            EXPR_APP => NodeKind::EXPR_ATOM.first(),
            EXPR_LET => TokenKind::KW_LET.into(),
            EXPR_PAREN => TokenKind::LPAREN.into(),
            EXPR_VAR => TokenKind::ID_EXPR.into(),
            EXPR_UNIT => TokenKind::KW_UNIT.into(),
            EXPR_META => TokenKindSet::empty(),
            BINDER => NAME.first(),
            NAME => TokenKind::ID_EXPR.into(),
            TYPE_ARROW => NodeKind::TYPE_ATOM.first(),
            TYPE_PAREN => TokenKind::LPAREN.into(),
            TYPE_VAR => TokenKind::ID_TYPE.into(),
            TYPE_UNIT => TokenKind::TY_UNIT.into(),
            TYPE_META => TokenKindSet::empty(),
            ERROR => panic!("NodeKind::ERROR.first() must not be called"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::Result, syntax::TRIVIA, Parser};

    #[allow(dead_code)]
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
    fn node_kind_first_terminates() {
        for node in NodeKindSet::all() {
            if node != ERROR {
                node.first();
            }
        }
    }

    #[test]
    fn node_kind_first_matches() {
        let cases: Vec<(
            NodeKind,
            fn(&mut Parser<'static>, TokenKindSet) -> Result<()>,
        )> = vec![
            (EXPR_ABS, Parser::expr_abs),
            (EXPR_APP, Parser::expr_app),
            (EXPR_LET, Parser::expr_let),
            (EXPR_PAREN, Parser::expr_paren),
            (EXPR_VAR, Parser::expr_var),
            (EXPR_UNIT, Parser::expr_unit),
            (BINDER, Parser::binder),
            (TYPE_ARROW, Parser::type_arrow),
            (TYPE_PAREN, Parser::type_paren),
            (TYPE_VAR, Parser::type_var),
            (TYPE_UNIT, Parser::type_unit),
        ];

        for (node, rule) in cases {
            assert_eq!(
                compute_first(rule),
                node.first(),
                "implemented vs declared FIRST({:?})",
                node
            );
        }
    }

    #[test]
    fn node_kind_set_first_matches() {
        let cases: Vec<(
            NodeKindSet,
            fn(&mut Parser<'static>, TokenKindSet) -> Result<()>,
        )> = vec![
            (NodeKind::EXPR, Parser::expr),
            (NodeKind::EXPR_ATOM, Parser::expr_atom),
            (NodeKind::TYPE, Parser::r#type),
            (NodeKind::TYPE_ATOM, Parser::type_atom),
        ];

        for (node_set, rule) in cases {
            assert_eq!(
                compute_first(rule),
                node_set.first(),
                "implemented vs declared FIRST({:?})",
                node_set
            );
        }
    }
}
