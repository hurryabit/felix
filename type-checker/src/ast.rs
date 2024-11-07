use std::{any::Any, rc::Rc};
use trait_gen::trait_gen;

use crate::Type;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident(Rc<String>);

#[derive(Clone, Debug)]
pub struct Binder {
    pub name: Ident,
    pub annot: Option<Type>,
}

#[derive(Clone, Debug)]
pub struct Expr(Rc<dyn Any>);

trait ExprNode: 'static {
    fn sub_exprs(&self) -> Vec<&Expr>;
}

#[derive(Clone, Debug)]
pub struct Broken;

#[derive(Clone, Debug)]
pub struct Var {
    pub name: Ident,
}

#[derive(Clone, Debug)]
pub struct Abs {
    pub binder: Binder,
    pub body: Expr,
}

#[derive(Clone, Debug)]
pub struct App {
    pub fun: Expr,
    pub arg: Expr,
}

#[derive(Clone, Debug)]
pub struct Let {
    pub binder: Binder,
    pub bindee: Expr,
    pub body: Expr,
}

#[derive(Clone, Debug)]
pub struct Unit;

pub trait FromExpr: Sized {
    fn from_expr(expr: &Expr) -> Option<Rc<Self>>;
}

#[trait_gen(T -> Broken, Var, Abs, App, Let, Unit)]
impl FromExpr for T {
    #[inline]
    fn from_expr(expr: &Expr) -> Option<Rc<Self>> {
        match expr.clone().0.downcast() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}

trait HasAnnot {
    fn annot(&self) -> &Option<Type>;
}

#[derive(Debug)]
pub struct Annot<const B: bool, T> {
    pub inner: Rc<T>,
    pub annot: Option<Type>,
}

impl<T> Annot<true, T> {
    pub fn annot(&self) -> &Type {
        self.annot
            .as_ref()
            .expect("annot should be Some when B = true")
    }
}

#[trait_gen(T -> Abs, Let)]
impl HasAnnot for T {
    fn annot(&self) -> &Option<Type> {
        &self.binder.annot
    }
}

impl<const B: bool, T: FromExpr + HasAnnot> FromExpr for Annot<B, T> {
    fn from_expr(expr: &Expr) -> Option<Rc<Self>> {
        let inner = T::from_expr(expr)?;
        let annot = inner.annot().as_ref().map(Type::clone);
        if annot.is_some() == B {
            Some(Rc::new(Annot { inner, annot }))
        } else {
            None
        }
    }
}

pub fn ident(name: &str) -> Ident {
    Ident(Rc::new(String::from(name)))
}

pub fn binder(name: &str) -> Binder {
    let name = ident(name);
    let annot = None;
    Binder { name, annot }
}

pub fn binder_annot(name: &str, r#type: Type) -> Binder {
    let name = ident(name);
    let annot = Some(r#type);
    Binder { name, annot }
}

pub fn broken() -> Expr {
    Expr(Rc::new(Broken))
}

pub fn var(name: &str) -> Expr {
    let name = ident(name);
    Expr(Rc::new(Var { name }))
}

pub fn abs(binder: Binder, body: Expr) -> Expr {
    Expr(Rc::new(Abs { binder, body }))
}

pub fn app(fun: Expr, arg: Expr) -> Expr {
    Expr(Rc::new(App { fun, arg }))
}

pub fn let_(binder: Binder, bindee: Expr, body: Expr) -> Expr {
    Expr(Rc::new(Let {
        binder,
        bindee,
        body,
    }))
}

pub fn unit() -> Expr {
    Expr(Rc::new(Unit))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn from_expr_annot_abs() {
        let abs_unannot = abs(binder("x"), broken());
        let abs_annot = abs(binder_annot("x", Type::Unit), broken());

        assert_matches!(
            <Annot<false, Abs>>::from_expr(&abs_unannot).as_deref(),
            Some(Annot { annot: None, .. })
        );
        assert_matches!(<Annot<true, Abs>>::from_expr(&abs_unannot), None);
        assert_matches!(<Annot<false, Abs>>::from_expr(&abs_annot), None);
        assert_matches!(
            <Annot<true, Abs>>::from_expr(&abs_annot).as_deref(),
            Some(Annot { annot: Some(_), .. })
        );
    }
}
