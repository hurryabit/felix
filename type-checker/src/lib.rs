#![allow(dead_code, unused_variables)]
use std::rc::Rc;

mod typ {
    use std::{fmt::Display, rc::Rc};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Type {
        Var(&'static str),
        Arrow(Rc<Type>, Rc<Type>),
        Unit,
    }

    impl Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Type::Var(name) => write!(f, "{}", name),
                Type::Arrow(param, res) => write!(f, "({} -> {})", param, res),
                Type::Unit => write!(f, "Unit"),
            }
        }
    }

    pub fn tvar(name: &'static str) -> Type {
        Type::Var(name)
    }

    pub fn arrow(param: Type, res: Type) -> Type {
        Type::Arrow(Rc::new(param), Rc::new(res))
    }

    pub const UNIT: Type = Type::Unit;
}

pub use typ::Type;

mod cst {
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

    pub fn abs(binder: &str, typ: super::Type, body: Node) -> Node {
        node(
            EXPR_ABS,
            vec![
                token(KW_LAM, "λ"),
                token(IDENT_EXPR, binder),
                token(PU_COLON, ":"),
                Child::Type(typ),
                token(PU_DOT, "."),
                Child::Node(body),
            ],
        )
    }

    pub fn app(fun: Node, arg: Node) -> Node {
        node(EXPR_APP, vec![Child::Node(fun), Child::Node(arg)])
    }

    pub fn let_(binder: &str, bindee: Node, body: Node) -> Node {
        node(
            EXPR_LET,
            vec![
                token(KW_LET, "let"),
                token(IDENT_EXPR, binder),
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
}

trait Pattern: TryFrom<cst::Node, Error = cst::Node> {
    const KIND: cst::NodeKind;
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Ident(Rc<String>);

impl From<&str> for Ident {
    fn from(name: &str) -> Self {
        Self(Rc::new(String::from(name)))
    }
}

struct Broken {
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

impl Pattern for Broken {
    const KIND: cst::NodeKind = cst::NodeKind::BROKEN;
}

struct Var {
    node: cst::Node,
    name: Ident,
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

struct Abs {
    node: cst::Node,
    binder: Ident,
    typ: Type,
    body: cst::Node,
}

impl TryFrom<cst::Node> for Abs {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_LAM,
            ..
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::IDENT_EXPR,
            value: binder,
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::PU_COLON,
            ..
        }), cst::Child::Type(typ), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::PU_DOT,
            ..
        }), cst::Child::Node(body)] = &node.children[..]
        {
            Ok(Abs {
                binder: Ident(Rc::clone(binder)),
                typ: typ.clone(),
                body: body.clone(),
                node,
            })
        } else {
            Err(node)
        }
    }
}

impl Pattern for Abs {
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_ABS;
}

struct App {
    node: cst::Node,
    fun: cst::Node,
    arg: cst::Node,
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

struct Let {
    node: cst::Node,
    binder: Ident,
    bindee: cst::Node,
    body: cst::Node,
}

impl TryFrom<cst::Node> for Let {
    type Error = cst::Node;

    fn try_from(node: cst::Node) -> std::result::Result<Self, Self::Error> {
        if node.kind != Self::KIND {
            return Err(node);
        }
        if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_LET,
            ..
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::IDENT_EXPR,
            value: binder,
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::PU_EQ,
            ..
        }), cst::Child::Node(bindee), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::KW_IN,
            ..
        }), cst::Child::Node(body)] = &node.children[..]
        {
            Ok(Let {
                binder: Ident(Rc::clone(binder)),
                bindee: bindee.clone(),
                body: body.clone(),
                node,
            })
        } else {
            Err(node)
        }
    }
}

impl Pattern for Let {
    const KIND: cst::NodeKind = cst::NodeKind::EXPR_LET;
}

struct Unit {
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

enum ContextData {
    Empty,
    Binding {
        ident: Ident,
        typ: Type,
        next: Context,
    },
}

#[derive(Clone)]
struct Context(Rc<ContextData>);

impl Context {
    fn new() -> Self {
        Self(Rc::new(ContextData::Empty))
    }

    fn lookup(&self, ident: &Ident) -> Option<Type> {
        match self.0.as_ref() {
            ContextData::Empty => None,
            ContextData::Binding {
                ident: bound,
                typ,
                next,
            } => {
                if ident == bound {
                    Some(typ.clone())
                } else {
                    next.lookup(ident)
                }
            }
        }
    }

    fn extend(&self, ident: Ident, typ: Type) -> Self {
        Self(Rc::new(ContextData::Binding {
            ident,
            typ,
            next: self.clone(),
        }))
    }
}

#[derive(Debug)]
enum TypeError {
    BrokenNode(cst::Node),
    UnknownEVar(Ident),
    NoInferRule(cst::Node),
    ExpectedArrow { found: Type },
    TypeMismatch { found: Type, expected: Type },
}

type Result<T> = std::result::Result<T, TypeError>;

trait Checker {
    fn lookup(&self, ctx: &Context, evar: &Ident) -> Result<Type>;
    fn check(&self, ctx: &Context, node: cst::Node, typ: Type) -> Result<()>;
    fn infer(&self, ctx: &Context, node: cst::Node) -> Result<Type>;
    fn equal(&self, found: &Type, expected: &Type) -> Result<()>;
    fn decompose_arrow(&self, typ: &Type) -> Result<(Type, Type)>;
}

struct InferRule {
    name: &'static str,
    kind: cst::NodeKind,
    rule: Box<
        dyn for<'a> Fn(
            &dyn Checker,
            &Context,
            cst::Node,
        ) -> std::result::Result<Result<Type>, cst::Node>,
    >,
}

impl InferRule {
    fn new<P: Pattern + 'static>(
        name: &'static str,
        rule: fn(&dyn Checker, &Context, P) -> Result<Type>,
    ) -> Self {
        Self {
            name,
            kind: P::KIND,
            rule: Box::new(
                move |checker: &dyn Checker, ctx: &Context, node: cst::Node| {
                    node.try_into().map(|pattern| rule(checker, ctx, pattern))
                },
            ),
        }
    }
}

struct TypeSystem {
    infer_rules: Vec<InferRule>,
}

impl TypeSystem {
    fn new() -> Self {
        Self {
            infer_rules: Vec::new(),
        }
    }

    fn add_infer_rule<P: Pattern + 'static>(
        &mut self,
        name: &'static str,
        rule: fn(&dyn Checker, &Context, P) -> Result<Type>,
    ) {
        self.infer_rules.push(InferRule::new::<P>(name, rule))
    }
}

impl Checker for TypeSystem {
    fn lookup(&self, ctx: &Context, evar: &Ident) -> Result<Type> {
        if let Some(typ) = ctx.lookup(&evar) {
            Ok(typ)
        } else {
            Err(TypeError::UnknownEVar(evar.clone()))
        }
    }

    fn check(&self, ctx: &Context, node: cst::Node, typ: Type) -> Result<()> {
        todo!()
    }

    fn infer(&self, ctx: &Context, mut node: cst::Node) -> Result<Type> {
        for rule in &self.infer_rules {
            match (rule.rule)(self, ctx, node) {
                Err(node1) => node = node1,
                Ok(res) => return res,
            }
        }
        Err(TypeError::NoInferRule(node))
    }

    fn equal(&self, found: &Type, expected: &Type) -> Result<()> {
        match (found, expected) {
            // TODO: We need to make sure the variables refer to the same binder.
            (typ::Type::Var(name1), typ::Type::Var(name2)) if name1 == name2 => Ok(()),
            (typ::Type::Arrow(found1, found2), typ::Type::Arrow(expected1, expected2)) => {
                self.equal(found1, expected1)?;
                self.equal(found2, expected2)
            }
            (typ::Type::Unit, typ::Type::Unit) => Ok(()),
            _ => Err(TypeError::TypeMismatch {
                found: found.clone(),
                expected: expected.clone(),
            }),
        }
    }

    fn decompose_arrow(&self, typ: &Type) -> Result<(Type, Type)> {
        match typ {
            typ::Type::Arrow(param, res) => Ok((param.as_ref().clone(), res.as_ref().clone())),
            _ => Err(TypeError::ExpectedArrow { found: typ.clone() }),
        }
    }
}

mod stlc {
    use super::*;

    fn t_broken(checker: &dyn Checker, ctx: &Context, broken: Broken) -> Result<Type> {
        Err(TypeError::BrokenNode(broken.node))
    }

    fn t_var(checker: &dyn Checker, ctx: &Context, var: Var) -> Result<Type> {
        checker.lookup(ctx, &var.name)
    }

    fn t_abs(checker: &dyn Checker, ctx: &Context, abs: Abs) -> Result<Type> {
        let ctx = ctx.extend(abs.binder, abs.typ.clone());
        let t_res = checker.infer(&ctx, abs.body)?;
        Ok(typ::arrow(abs.typ, t_res))
    }

    fn t_app(checker: &dyn Checker, ctx: &Context, app: App) -> Result<Type> {
        let t_fun = checker.infer(ctx, app.fun)?;
        let (t_param, t_res) = checker.decompose_arrow(&t_fun)?;
        let t_arg = checker.infer(ctx, app.arg)?;
        checker.equal(&t_arg, &t_param)?;
        Ok(t_res)
    }

    fn t_let(checker: &dyn Checker, ctx: &Context, let_: Let) -> Result<Type> {
        let t1 = checker.infer(ctx, let_.bindee)?;
        let ctx1 = ctx.extend(let_.binder, t1);
        checker.infer(&ctx1, let_.body)
    }

    fn t_unit(checker: &dyn Checker, ctx: &Context, _unit: Unit) -> Result<Type> {
        Ok(typ::UNIT)
    }

    pub fn make() -> TypeSystem {
        let mut ts = TypeSystem::new();
        ts.add_infer_rule("T-Broken", t_broken);
        ts.add_infer_rule("T-Var", t_var);
        ts.add_infer_rule("T-Abs", t_abs);
        ts.add_infer_rule("T-App", t_app);
        ts.add_infer_rule("T-Let", t_let);
        ts.add_infer_rule("T-Unit", t_unit);
        ts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cst::*;
    use typ::*;

    use assert_matches::assert_matches;

    impl std::ops::Shr<Type> for Type {
        type Output = Type;

        fn shr(self, rhs: Type) -> Self::Output {
            arrow(self, rhs)
        }
    }

    #[test]
    fn pattern_broken() {
        assert!(Broken::try_from(broken()).is_ok());
    }

    #[test]
    fn pattern_var() {
        assert!(Var::try_from(var("x")).is_ok());
    }

    #[test]
    fn pattern_abs() {
        assert!(Abs::try_from(abs("x", tvar("T"), var("E"))).is_ok());
    }

    #[test]
    fn pattern_app() {
        assert!(App::try_from(app(var("F"), var("A"))).is_ok());
    }

    #[test]
    fn pattern_unit() {
        assert!(Unit::try_from(unit()).is_ok());
    }

    #[test]
    fn rule_t_broken() {
        let ctx = Context::new();
        let res = stlc::make().infer(&ctx, broken());
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn rule_t_var_ok() {
        let ctx = Context::new().extend(Ident::from("x"), tvar("T"));
        let res = stlc::make().infer(&ctx, var("x"));
        assert_eq!(res.unwrap(), tvar("T"));
    }

    #[test]
    fn rule_t_var_unknown() {
        let res = stlc::make().infer(&Context::new(), var("x"));
        assert_matches!(res, Err(TypeError::UnknownEVar(_)));
    }

    #[test]
    fn rule_t_abs_ok() {
        let ctx = Context::new().extend(Ident::from("E"), tvar("S"));
        let res = stlc::make().infer(&ctx, abs("x", tvar("T"), var("E")));
        assert_eq!(res.unwrap(), tvar("T") >> tvar("S"));
    }

    #[test]
    fn rule_t_abs_type_propagates() {
        let res = stlc::make().infer(&Context::new(), abs("x", tvar("T"), var("x")));
        assert_eq!(res.unwrap(), tvar("T") >> tvar("T"));
    }

    #[test]
    fn rule_t_abs_error_propagates() {
        let res = stlc::make().infer(&Context::new(), abs("x", tvar("T"), broken()));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn rule_t_app_ok() {
        let ctx = Context::new()
            .extend(Ident::from("F"), tvar("S") >> tvar("T"))
            .extend(Ident::from("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, app(var("F"), var("A")));
        assert_eq!(res.unwrap(), tvar("T"));
    }

    #[test]
    fn rule_t_app_no_arrow() {
        let ctx = Context::new()
            .extend(Ident::from("F"), tvar("T"))
            .extend(Ident::from("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, app(var("F"), var("X")));
        assert_matches!(res, Err(TypeError::ExpectedArrow { .. }));
    }

    #[test]
    fn rule_t_app_mismatch() {
        let ctx = Context::new()
            .extend(Ident::from("F"), tvar("S") >> tvar("T"))
            .extend(Ident::from("A"), tvar("U"));
        let res = stlc::make().infer(&ctx, app(var("F"), var("A")));
        assert_matches!(res, Err(TypeError::TypeMismatch { .. }));
    }

    #[test]
    fn rule_t_app_error_propagates_fun() {
        let ctx = Context::new().extend(Ident::from("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, app(broken(), var("A")));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn rule_t_error_propagates_arg() {
        let ctx = Context::new().extend(Ident::from("F"), tvar("S") >> tvar("T"));
        let res = stlc::make().infer(&ctx, app(var("F"), broken()));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn rule_t_let_ok() {
        let ctx = Context::new()
            .extend(Ident::from("A"), tvar("S"))
            .extend(Ident::from("B"), tvar("T"));
        let res = stlc::make().infer(&ctx, let_("x", var("A"), var("B")));
        assert_eq!(res.unwrap(), tvar("T"));
    }

    #[test]
    fn rule_t_let_type_propagates() {
        let ctx = Context::new().extend(Ident::from("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, let_("x", var("A"), var("x")));
        assert_eq!(res.unwrap(), tvar("S"));
    }

    #[test]
    fn rule_t_let_not_rec() {
        let ctx = Context::new().extend(Ident::from("B"), tvar("T"));
        let res = stlc::make().infer(&ctx, let_("x", var("x"), var("B")));
        assert_matches!(res, Err(TypeError::UnknownEVar(_)));
    }

    #[test]
    fn rule_t_let_error_propagates_bindee() {
        let ctx = Context::new().extend(Ident::from("B"), tvar("T"));
        let res = stlc::make().infer(&ctx, let_("x", broken(), var("B")));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn rule_t_let_error_propagates_body() {
        let ctx = Context::new().extend(Ident::from("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, let_("x", var("A"), broken()));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn rule_t_unit() {
        let res = stlc::make().infer(&Context::new(), unit());
        assert_eq!(res.unwrap(), UNIT);
    }
}
