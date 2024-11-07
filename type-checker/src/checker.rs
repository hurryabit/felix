use std::{borrow::Borrow, rc::Rc};

use crate::{
    ast::{self, Expr, FromExpr, Ident},
    Type,
};

enum ContextData {
    Empty,
    Binding {
        ident: Ident,
        r#type: Type,
        next: Context,
    },
}

#[derive(Clone)]
pub struct Context(Rc<ContextData>);

impl Context {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Rc::new(ContextData::Empty))
    }

    fn lookup(&self, ident: &Ident) -> Option<Type> {
        match self.0.as_ref() {
            ContextData::Empty => None,
            ContextData::Binding {
                ident: bound,
                r#type,
                next,
            } => {
                if ident == bound {
                    Some(r#type.clone())
                } else {
                    next.lookup(ident)
                }
            }
        }
    }

    pub fn extend(&self, ident: Ident, r#type: Type) -> Self {
        Self(Rc::new(ContextData::Binding {
            ident,
            r#type,
            next: self.clone(),
        }))
    }
}

#[derive(Debug)]
pub enum TypeError {
    BrokenNode(Rc<ast::Broken>),
    UnknownEVar(Ident),
    NoInferRule(Expr),
    ExpectedArrow { found: Type },
    TypeMismatch { found: Type, expected: Type },
}

pub type Result<T> = std::result::Result<T, TypeError>;

pub trait Checker {
    fn lookup(&self, ctx: &Context, evar: &Ident) -> Result<Type>;
    fn check(&self, ctx: &Context, expr: &Expr, r#type: Type) -> Result<()>;
    fn infer(&self, ctx: &Context, expr: &Expr) -> Result<Type>;
    fn equal(&self, found: &Type, expected: &Type) -> Result<()>;
    fn decompose_arrow(&self, r#type: &Type) -> Result<(Type, Type)>;
}

struct InferRule {
    name: &'static str,
    #[allow(clippy::type_complexity)]
    rule: Box<dyn Fn(&dyn Checker, &Context, &Expr) -> Option<Result<Type>> + Send + Sync>,
}

impl InferRule {
    fn new<T: FromExpr + 'static>(
        name: &'static str,
        rule: fn(&dyn Checker, &Context, &Rc<T>) -> Result<Type>,
    ) -> Self {
        Self {
            name,
            rule: Box::new(move |checker: &dyn Checker, ctx: &Context, expr: &Expr| {
                T::from_expr(expr).map(|pattern| rule(checker, ctx, pattern.borrow()))
            }),
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

    pub fn add_infer_rule<T: FromExpr + 'static>(
        &mut self,
        name: &'static str,
        rule: fn(&dyn Checker, &Context, &Rc<T>) -> Result<Type>,
    ) {
        self.infer_rules.push(InferRule::new::<T>(name, rule))
    }
}

impl Checker for TypeSystem {
    fn lookup(&self, ctx: &Context, evar: &Ident) -> Result<Type> {
        if let Some(r#type) = ctx.lookup(evar) {
            Ok(r#type)
        } else {
            Err(TypeError::UnknownEVar(evar.clone()))
        }
    }

    #[allow(unused_variables)]
    fn check(&self, ctx: &Context, expr: &Expr, r#type: Type) -> Result<()> {
        todo!()
    }

    fn infer(&self, ctx: &Context, expr: &Expr) -> Result<Type> {
        for rule in &self.infer_rules {
            if let Some(res) = (rule.rule)(self, ctx, expr) {
                return res;
            }
        }
        Err(TypeError::NoInferRule(expr.clone()))
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

    fn decompose_arrow(&self, r#type: &Type) -> Result<(Type, Type)> {
        match r#type {
            Type::Arrow(param, res) => Ok((param.as_ref().clone(), res.as_ref().clone())),
            _ => Err(TypeError::ExpectedArrow {
                found: r#type.clone(),
            }),
        }
    }
}
