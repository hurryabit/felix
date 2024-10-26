use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenKind {
    IDENT_EXPR,
    KW_LAM,
    KW_LET,
    KW_IN,
    KW_UNIT,
    PU_COLON,
    PU_DOT,
    PU_EQ,
}

use TokenKind::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum NodeKind {
    BROKEN,
    BINDER,
    EXPR_VAR,
    EXPR_ABS,
    EXPR_APP,
    EXPR_LET,
    EXPR_UNIT,
}

use NodeKind::*;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Rc<String>,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub children: Rc<Vec<Child>>,
}

#[derive(Clone, Debug)]
pub enum Child {
    Token(Token),
    Node(Node),
    Type(super::Type), // TODO: Remove this shortcut.
}

fn token(kind: TokenKind, value: &str) -> Child {
    let value = Rc::new(String::from(value));
    Child::Token(Token { kind, value })
}

fn node(kind: NodeKind, children: Vec<Child>) -> Node {
    Node {
        kind,
        children: Rc::new(children),
    }
}

pub fn broken() -> Node {
    node(BROKEN, vec![])
}

pub fn var(name: &str) -> Node {
    node(EXPR_VAR, vec![token(IDENT_EXPR, name)])
}

pub fn abs(binder: Node, body: Node) -> Node {
    node(
        EXPR_ABS,
        vec![
            token(KW_LAM, "λ"),
            Child::Node(binder),
            token(PU_DOT, "."),
            Child::Node(body),
        ],
    )
}

pub fn app(fun: Node, arg: Node) -> Node {
    node(EXPR_APP, vec![Child::Node(fun), Child::Node(arg)])
}

pub fn let_(binder: Node, bindee: Node, body: Node) -> Node {
    node(
        EXPR_LET,
        vec![
            token(KW_LET, "let"),
            Child::Node(binder),
            token(PU_EQ, "="),
            Child::Node(bindee),
            token(KW_IN, "in"),
            Child::Node(body),
        ],
    )
}

pub fn unit() -> Node {
    node(EXPR_UNIT, vec![token(TokenKind::KW_UNIT, "unit")])
}

pub fn binder(name: &str) -> Node {
    node(BINDER, vec![token(IDENT_EXPR, name)])
}

pub fn binder_annot(name: &str, typ: super::Type) -> Node {
    node(
        BINDER,
        vec![
            token(IDENT_EXPR, name),
            token(PU_COLON, ":"),
            Child::Type(typ),
        ],
    )
}
