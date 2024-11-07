#![allow(dead_code)]
use crate::syntax::{
    Node,
    NodeKind::{self, *},
    TokenKind::{self, *},
};

pub struct GreenChild(rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>);

impl GreenChild {
    pub fn into_syntax(self) -> Node {
        Node::new_root(self.0.into_node().unwrap())
    }
}

pub fn expr_abs(binder: GreenChild, body: GreenChild) -> GreenChild {
    node(
        EXPR_ABS,
        vec![
            token(GR_LAMBDA_LOWER, "λ"),
            binder,
            token(DOT, "."),
            scope(body),
        ],
    )
}

pub fn expr_app(fun: GreenChild, arg: GreenChild) -> GreenChild {
    node(EXPR_APP, vec![fun, arg])
}

pub fn expr_let(binder: GreenChild, bindee: GreenChild, body: GreenChild) -> GreenChild {
    node(
        EXPR_LET,
        vec![
            token(KW_LET, "let"),
            binder,
            token(EQUALS, "="),
            bindee,
            token(KW_IN, "in"),
            scope(body),
        ],
    )
}

pub fn expr_paren(expr: GreenChild) -> GreenChild {
    node(
        EXPR_PAREN,
        vec![token(LPAREN, "("), expr, token(RPAREN, ")")],
    )
}

pub fn expr_var(name: &str) -> GreenChild {
    node(EXPR_VAR, vec![token(ID_EXPR, name)])
}

pub fn expr_unit() -> GreenChild {
    node(EXPR_UNIT, vec![token(KW_UNIT, "unit")])
}

pub fn expr_meta(name: &str) -> GreenChild {
    node(EXPR_META, vec![token(ID_EXPR, name)])
}

pub fn type_arrow(param: GreenChild, result: GreenChild) -> GreenChild {
    node(TYPE_ARROW, vec![param, token(OP_ARROW, "->"), result])
}

pub fn type_paren(r#type: GreenChild) -> GreenChild {
    node(
        TYPE_PAREN,
        vec![token(LPAREN, "("), r#type, token(RPAREN, ")")],
    )
}

pub fn type_var(name: &str) -> GreenChild {
    node(TYPE_VAR, vec![token(ID_TYPE, name)])
}

pub fn type_unit() -> GreenChild {
    node(TYPE_UNIT, vec![token(TY_UNIT, "Unit")])
}

pub fn type_meta(name: &str) -> GreenChild {
    node(TYPE_META, vec![token(ID_TYPE, name)])
}

pub fn binder(name: &str, r#type: Option<GreenChild>) -> GreenChild {
    match r#type {
        Some(r#type) => node(BINDER, vec![self::name(name), token(COLON, ":"), r#type]),
        None => node(BINDER, vec![self::name(name)]),
    }
}

fn name(text: &str) -> GreenChild {
    // TODO(MH): Assert that name is well-formed.
    node(NAME, vec![token(ID_EXPR, text)])
}

fn scope(body: GreenChild) -> GreenChild {
    node(SCOPE, vec![body])
}

fn token(kind: TokenKind, text: &str) -> GreenChild {
    GreenChild(rowan::NodeOrToken::Token(rowan::GreenToken::new(
        kind.into(),
        text,
    )))
}

fn node(kind: NodeKind, children: Vec<GreenChild>) -> GreenChild {
    GreenChild(rowan::NodeOrToken::Node(rowan::GreenNode::new(
        kind.into(),
        children.into_iter().map(|child| child.0),
    )))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[allow(unused_variables, unreachable_code)]
    #[ignore = "template"]
    #[test]
    fn syntax_template() {
        let syntax = expr_meta("e").into_syntax();
        assert_debug_snapshot!(syntax, @r"");
    }

    #[test]
    fn syntax_expr_abs_annot() {
        let syntax = expr_abs(binder("x", Some(type_meta("T"))), expr_meta("e")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_ABS@0..7
          GR_LAMBDA_LOWER@0..2 "λ"
          BINDER@2..5
            NAME@2..3
              ID_EXPR@2..3 "x"
            COLON@3..4 ":"
            TYPE_META@4..5
              ID_TYPE@4..5 "T"
          DOT@5..6 "."
          SCOPE@6..7
            EXPR_META@6..7
              ID_EXPR@6..7 "e"
        "#);
    }

    #[test]
    fn syntax_expr_abs_plain() {
        let syntax = expr_abs(binder("x", None), expr_meta("e")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_ABS@0..5
          GR_LAMBDA_LOWER@0..2 "λ"
          BINDER@2..3
            NAME@2..3
              ID_EXPR@2..3 "x"
          DOT@3..4 "."
          SCOPE@4..5
            EXPR_META@4..5
              ID_EXPR@4..5 "e"
        "#);
    }

    #[test]
    fn syntax_expr_app() {
        let syntax = expr_app(expr_meta("e1"), expr_meta("e2")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_APP@0..4
          EXPR_META@0..2
            ID_EXPR@0..2 "e1"
          EXPR_META@2..4
            ID_EXPR@2..4 "e2"
        "#);
    }

    #[test]
    fn syntax_expr_let_annot() {
        let syntax = expr_let(
            binder("x", Some(type_meta("T"))),
            expr_meta("e1"),
            expr_meta("e2"),
        )
        .into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_LET@0..13
          KW_LET@0..3 "let"
          BINDER@3..6
            NAME@3..4
              ID_EXPR@3..4 "x"
            COLON@4..5 ":"
            TYPE_META@5..6
              ID_TYPE@5..6 "T"
          EQUALS@6..7 "="
          EXPR_META@7..9
            ID_EXPR@7..9 "e1"
          KW_IN@9..11 "in"
          SCOPE@11..13
            EXPR_META@11..13
              ID_EXPR@11..13 "e2"
        "#);
    }

    #[test]
    fn syntax_expr_let_plain() {
        let syntax = expr_let(binder("x", None), expr_meta("e1"), expr_meta("e2")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_LET@0..11
          KW_LET@0..3 "let"
          BINDER@3..4
            NAME@3..4
              ID_EXPR@3..4 "x"
          EQUALS@4..5 "="
          EXPR_META@5..7
            ID_EXPR@5..7 "e1"
          KW_IN@7..9 "in"
          SCOPE@9..11
            EXPR_META@9..11
              ID_EXPR@9..11 "e2"
        "#);
    }

    #[test]
    fn syntax_expr_paren() {
        let syntax = expr_paren(expr_meta("e")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_PAREN@0..3
          LPAREN@0..1 "("
          EXPR_META@1..2
            ID_EXPR@1..2 "e"
          RPAREN@2..3 ")"
        "#);
    }

    #[test]
    fn syntax_expr_var() {
        let syntax = expr_var("x").into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_VAR@0..1
          ID_EXPR@0..1 "x"
        "#);
    }

    #[test]
    fn syntax_expr_unit() {
        let syntax = expr_unit().into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        EXPR_UNIT@0..4
          KW_UNIT@0..4 "unit"
        "#);
    }

    #[test]
    fn syntax_type_arrow() {
        let syntax = type_arrow(type_meta("T1"), type_meta("T2")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        TYPE_ARROW@0..6
          TYPE_META@0..2
            ID_TYPE@0..2 "T1"
          OP_ARROW@2..4 "->"
          TYPE_META@4..6
            ID_TYPE@4..6 "T2"
        "#);
    }

    #[test]
    fn syntax_type_paren() {
        let syntax = type_paren(type_meta("T")).into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        TYPE_PAREN@0..3
          LPAREN@0..1 "("
          TYPE_META@1..2
            ID_TYPE@1..2 "T"
          RPAREN@2..3 ")"
        "#);
    }

    #[test]
    fn syntax_type_var() {
        let syntax = type_var("A").into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        TYPE_VAR@0..1
          ID_TYPE@0..1 "A"
        "#);
    }

    #[test]
    fn syntax_type_unit() {
        let syntax = type_unit().into_syntax();
        assert_debug_snapshot!(syntax, @r#"
        TYPE_UNIT@0..4
          TY_UNIT@0..4 "Unit"
        "#);
    }
}
