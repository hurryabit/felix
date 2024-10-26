use std::rc::Rc;

use crate::{
    cst,
    tl::{self, AsOption},
    Type,
};

pub trait Pattern: TryFrom<cst::Node, Error = cst::Node> {
    const KIND: cst::NodeKind;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident(Rc<String>);

impl From<&str> for Ident {
    fn from(name: &str) -> Self {
        Self(Rc::new(name.into()))
    }
}

impl From<&Rc<String>> for Ident {
    fn from(rc: &Rc<String>) -> Self {
        Self(Rc::clone(rc))
    }
}

pub struct Broken {
    node: cst::Node,
}

impl TryFrom<cst::Node> for Broken {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        Ok(Broken { node })
    }
}

impl Into<cst::Node> for Broken {
    fn into(self) -> cst::Node {
        self.node
    }
}

impl Pattern for Broken {
    const KIND: cst::NodeKind = cst::NodeKind::BROKEN;
}

pub struct Binder<Ann: tl::Bool> {
    node: cst::Node,
    pub name: Ident,
    pub typ: Ann::Option<Type>,
}

impl TryFrom<cst::Node> for Binder<tl::False> {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            Err(node)
        } else if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::IDENT_EXPR,
            value: name,
        })] = &node.children[..]
        {
            Ok(Self {
                name: Ident::from(name),
                typ: tl::None(),
                node,
            })
        } else {
            Err(node)
        }
    }
}

impl Pattern for Binder<tl::False> {
    const KIND: cst::NodeKind = cst::NodeKind::BINDER;
}

impl TryFrom<cst::Node> for Binder<tl::True> {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            Err(node)
        } else if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::IDENT_EXPR,
            value: name,
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::PU_COLON,
            ..
        }), cst::Child::Type(typ)] = &node.children[..]
        {
            Ok(Self {
                name: Ident::from(name),
                typ: tl::Some(typ.clone()),
                node,
            })
        } else {
            Err(node)
        }
    }
}

impl Pattern for Binder<tl::True> {
    const KIND: cst::NodeKind = cst::NodeKind::BINDER;
}

impl TryFrom<cst::Node> for Binder<tl::Unknown> {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        match Binder::<tl::False>::try_from(node) {
            Ok(binder) => Ok(Self {
                node: binder.node,
                name: binder.name,
                typ: binder.typ.as_option(),
            }),
            Err(node) => match Binder::<tl::True>::try_from(node) {
                Ok(binder) => Ok(Self {
                    node: binder.node,
                    name: binder.name,
                    typ: binder.typ.as_option(),
                }),
                Err(node) => Err(node),
            },
        }
    }
}

impl Pattern for Binder<tl::Unknown> {
    const KIND: cst::NodeKind = cst::NodeKind::BINDER;
}

pub struct Var {
    node: cst::Node,
    pub name: Ident,
}

impl TryFrom<cst::Node> for Var {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            Err(node)
        } else if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::IDENT_EXPR,
            value: name,
        })] = &node.children[..]
        {
            Ok(Var {
                name: Ident(Rc::clone(name)),
                node,
            })
        } else {
            Err(node)
        }
    }
}

impl Pattern for Var {
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_VAR;
}

pub struct Abs<Ann: tl::Bool> {
    node: cst::Node,
    pub binder: Binder<Ann>,
    pub body: cst::Node,
}

impl<Ann: tl::Bool> TryFrom<cst::Node> for Abs<Ann>
where
    Binder<Ann>: TryFrom<cst::Node>,
{
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_LAM,
            ..
        }), cst::Child::Node(binder), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::PU_DOT,
            ..
        }), cst::Child::Node(body)] = &node.children[..]
        {
            if let Ok(binder) = Binder::try_from(binder.clone()) {
                return Ok(Self {
                    binder,
                    body: body.clone(),
                    node,
                });
            }
        }
        Err(node)
    }
}

impl<Ann: tl::Bool> Pattern for Abs<Ann>
where
    Binder<Ann>: TryFrom<cst::Node>,
{
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_ABS;
}

pub struct App {
    node: cst::Node,
    pub fun: cst::Node,
    pub arg: cst::Node,
}

impl TryFrom<cst::Node> for App {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        if let [cst::Child::Node(fun), cst::Child::Node(arg)] = &node.children[..] {
            Ok(App {
                fun: fun.clone(),
                arg: arg.clone(),
                node,
            })
        } else {
            Err(node)
        }
    }
}

impl Pattern for App {
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_APP;
}

pub struct Let<Ann: tl::Bool> {
    node: cst::Node,
    pub binder: Binder<Ann>,
    pub bindee: cst::Node,
    pub body: cst::Node,
}

impl<Ann: tl::Bool> TryFrom<cst::Node> for Let<Ann>
where
    Binder<Ann>: TryFrom<cst::Node>,
{
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_LET,
            ..
        }), cst::Child::Node(binder), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::PU_EQ,
            ..
        }), cst::Child::Node(bindee), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_IN,
            ..
        }), cst::Child::Node(body)] = &node.children[..]
        {
            if let Ok(binder) = Binder::try_from(binder.clone()) {
                return Ok(Let {
                    binder,
                    bindee: bindee.clone(),
                    body: body.clone(),
                    node,
                });
            }
        }
        Err(node)
    }
}

impl<Ann: tl::Bool> Pattern for Let<Ann>
where
    Binder<Ann>: TryFrom<cst::Node>,
{
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_LET;
}

pub struct Unit {
    node: cst::Node,
}

impl TryFrom<cst::Node> for Unit {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_UNIT,
            ..
        })] = &node.children[..]
        {
            Ok(Unit { node })
        } else {
            Err(node)
        }
    }
}

impl Pattern for Unit {
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_UNIT;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cst::*, typ::tvar};

    #[test]
    fn broken_matches() {
        assert!(Broken::try_from(broken()).is_ok());
    }

    #[test]
    fn binder_unannot_matches_false() {
        assert!(Binder::<tl::False>::try_from(binder("x")).is_ok());
    }

    #[test]
    fn binder_annot_not_matches_false() {
        assert!(Binder::<tl::False>::try_from(binder_annot("x", tvar("T"))).is_err());
    }

    #[test]
    fn binder_unannot_not_matches_true() {
        assert!(Binder::<tl::True>::try_from(binder("x")).is_err());
    }

    #[test]
    fn binder_annot_matches_true() {
        assert!(Binder::<tl::True>::try_from(binder_annot("x", tvar("T"))).is_ok());
    }

    fn binder_unannot_matches_unknown() {
        assert!(Binder::<tl::Unknown>::try_from(binder("x")).is_ok());
    }

    #[test]
    fn binder_annot_matches_unknown() {
        assert!(Binder::<tl::Unknown>::try_from(binder_annot("x", tvar("T"))).is_ok());
    }

    #[test]
    fn var_matches() {
        assert!(Var::try_from(var("x")).is_ok());
    }

    #[test]
    fn abs_annot_matches() {
        assert!(Abs::<tl::True>::try_from(abs(binder_annot("x", tvar("T")), var("E"))).is_ok());
    }

    #[test]
    fn abs_unannot_matches() {
        assert!(Abs::<tl::False>::try_from(abs(binder("x"), var("E"))).is_ok());
    }

    #[test]
    fn app_matches() {
        assert!(App::try_from(app(var("F"), var("A"))).is_ok());
    }

    #[test]
    fn let_unannot_matches() {
        assert!(Let::<tl::False>::try_from(let_(binder("x"), var("A"), var("B"))).is_ok());
    }

    #[test]
    fn let_annot_matches() {
        assert!(
            Let::<tl::True>::try_from(let_(binder_annot("x", tvar("T")), var("A"), var("B")))
                .is_ok()
        );
    }

    #[test]
    fn unit_matches() {
        assert!(Unit::try_from(unit()).is_ok());
    }
}
