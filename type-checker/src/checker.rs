use std::rc::Rc;

use crate::{
    ast::{Ident, Pattern},
    cst, Type,
};

enum ContextData {
    Empty,
    Binding {
        ident: Ident,
        typ: Type,
        next: Context,
    },
}

#[derive(Clone)]
pub struct Context(Rc<ContextData>);

impl Context {
    pub fn new() -> Self {
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

    pub fn extend(&self, ident: Ident, typ: Type) -> Self {
        Self(Rc::new(ContextData::Binding {
            ident,
            typ,
            next: self.clone(),
        }))
    }
}

#[derive(Debug)]
pub enum TypeError {
    BrokenNode(cst::Node),
    UnknownEVar(Ident),
    NoInferRule(cst::Node),
    ExpectedArrow { found: Type },
    TypeMismatch { found: Type, expected: Type },
}

pub type Result<T> = std::result::Result<T, TypeError>;

pub trait Checker {
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
        dyn Fn(&dyn Checker, &Context, cst::Node) -> std::result::Result<Result<Type>, cst::Node>
            + Send
            + Sync,
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

pub struct TypeSystem {
    pub name: String,
    infer_rules: Vec<InferRule>,
}

impl TypeSystem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            infer_rules: Vec::new(),
        }
    }

    pub fn add_infer_rule<P: Pattern + 'static>(
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
            (Type::Var(name1), Type::Var(name2)) if name1 == name2 => Ok(()),
            (Type::Arrow(found1, found2), Type::Arrow(expected1, expected2)) => {
                self.equal(found1, expected1)?;
                self.equal(found2, expected2)
            }
            (Type::Unit, Type::Unit) => Ok(()),
            _ => Err(TypeError::TypeMismatch {
                found: found.clone(),
                expected: expected.clone(),
            }),
        }
    }

    fn decompose_arrow(&self, typ: &Type) -> Result<(Type, Type)> {
        match typ {
            Type::Arrow(param, res) => Ok((param.as_ref().clone(), res.as_ref().clone())),
            _ => Err(TypeError::ExpectedArrow { found: typ.clone() }),
        }
    }
}
