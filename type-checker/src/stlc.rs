use std::{rc::Rc, sync::LazyLock};

use crate::*;
use ast::*;

fn t_broken(_checker: &dyn Checker, _ctx: &Context, broken: &Rc<Broken>) -> Result<Type> {
    Err(TypeError::BrokenNode(broken.clone()))
}

fn t_var(checker: &dyn Checker, ctx: &Context, var: &Rc<Var>) -> Result<Type> {
    checker.lookup(ctx, &var.name)
}

fn t_abs(checker: &dyn Checker, ctx: &Context, abs: &Rc<Annot<true, Abs>>) -> Result<Type> {
    let t_binder = abs.annot();
    let ctx = ctx.extend(abs.inner.binder.name.clone(), t_binder.clone());
    let t_res = checker.infer(&ctx, &abs.inner.body)?;
    Ok(typ::arrow(t_binder.clone(), t_res))
}

fn t_app(checker: &dyn Checker, ctx: &Context, app: &Rc<App>) -> Result<Type> {
    let t_fun = checker.infer(ctx, &app.fun)?;
    let (t_param, t_res) = checker.decompose_arrow(&t_fun)?;
    let t_arg = checker.infer(ctx, &app.arg)?;
    checker.equal(&t_arg, &t_param)?;
    Ok(t_res)
}

fn t_let(checker: &dyn Checker, ctx: &Context, let_: &Rc<Annot<false, Let>>) -> Result<Type> {
    let let_ = &let_.inner;
    let t1 = checker.infer(ctx, &let_.bindee)?;
    let ctx1 = ctx.extend(let_.binder.name.clone(), t1);
    checker.infer(&ctx1, &let_.body)
}

fn t_unit(_checker: &dyn Checker, _ctx: &Context, _unit: &Rc<Unit>) -> Result<Type> {
    Ok(typ::UNIT)
}

fn make() -> TypeSystem {
    let mut ts = TypeSystem::new(String::from("Simply Typed Lambda Calculus"));
    ts.add_infer_rule("T-Broken", t_broken);
    ts.add_infer_rule("T-Var", t_var);
    ts.add_infer_rule("T-Abs", t_abs);
    ts.add_infer_rule("T-App", t_app);
    ts.add_infer_rule("T-Let", t_let);
    ts.add_infer_rule("T-Unit", t_unit);
    ts
}

static INSTANCE: LazyLock<TypeSystem> = LazyLock::new(make);

pub fn get() -> &'static TypeSystem {
    &INSTANCE
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;
    // use as::*;
    use typ::*;

    impl std::ops::Shr<Type> for Type {
        type Output = Type;

        fn shr(self, rhs: Type) -> Self::Output {
            arrow(self, rhs)
        }
    }

    #[test]
    fn t_broken() {
        let ctx = Context::new();
        let res = stlc::make().infer(&ctx, &broken());
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn t_var_ok() {
        let ctx = Context::new().extend(ident("x"), tvar("T"));
        let res = stlc::make().infer(&ctx, &var("x"));
        assert_eq!(res.unwrap(), tvar("T"));
    }

    #[test]
    fn t_var_unknown() {
        let res = stlc::make().infer(&Context::new(), &var("x"));
        assert_matches!(res, Err(TypeError::UnknownEVar(_)));
    }

    #[test]
    fn t_abs_ok() {
        let ctx = Context::new().extend(ident("E"), tvar("S"));
        let res = stlc::make().infer(&ctx, &abs(binder_annot("x", tvar("T")), var("E")));
        assert_eq!(res.unwrap(), tvar("T") >> tvar("S"));
    }

    #[test]
    fn t_abs_type_propagates() {
        let res = stlc::make().infer(&Context::new(), &abs(binder_annot("x", tvar("T")), var("x")));
        assert_eq!(res.unwrap(), tvar("T") >> tvar("T"));
    }

    #[test]
    fn t_abs_error_propagates() {
        let res = stlc::make().infer(&Context::new(), &abs(binder_annot("x", tvar("T")), broken()));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn t_abs_no_annot() {
        let res = stlc::make().infer(&Context::new(), &abs(binder("x"), broken()));
        assert_matches!(res, Err(TypeError::NoInferRule(_)));
    }

    #[test]
    fn t_app_ok() {
        let ctx = Context::new()
            .extend(ident("F"), tvar("S") >> tvar("T"))
            .extend(ident("A"), tvar("S"));
        let res = stlc::make().infer(&ctx,&app(var("F"), var("A")));
        assert_eq!(res.unwrap(), tvar("T"));
    }

    #[test]
    fn t_app_no_arrow() {
        let ctx = Context::new()
            .extend(ident("F"), tvar("T"))
            .extend(ident("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, &app(var("F"), var("X")));
        assert_matches!(res, Err(TypeError::ExpectedArrow { .. }));
    }

    #[test]
    fn t_app_mismatch() {
        let ctx = Context::new()
            .extend(ident("F"), tvar("S") >> tvar("T"))
            .extend(ident("A"), tvar("U"));
        let res = stlc::make().infer(&ctx, &app(var("F"), var("A")));
        assert_matches!(res, Err(TypeError::TypeMismatch { .. }));
    }

    #[test]
    fn t_app_error_propagates_fun() {
        let ctx = Context::new().extend(ident("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, &app(broken(), var("A")));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn t_error_propagates_arg() {
        let ctx = Context::new().extend(ident("F"), tvar("S") >> tvar("T"));
        let res = stlc::make().infer(&ctx, &app(var("F"), broken()));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn t_let_ok() {
        let ctx = Context::new()
            .extend(ident("A"), tvar("S"))
            .extend(ident("B"), tvar("T"));
        let res = stlc::make().infer(&ctx, &let_(binder("x"), var("A"), var("B")));
        assert_eq!(res.unwrap(), tvar("T"));
    }

    #[test]
    fn t_let_type_propagates() {
        let ctx = Context::new().extend(ident("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, &let_(binder("x"), var("A"), var("x")));
        assert_eq!(res.unwrap(), tvar("S"));
    }

    #[test]
    fn t_let_not_rec() {
        let ctx = Context::new().extend(ident("B"), tvar("T"));
        let res = stlc::make().infer(&ctx, &let_(binder("x"), var("x"), var("B")));
        assert_matches!(res, Err(TypeError::UnknownEVar(_)));
    }

    #[test]
    fn t_let_error_propagates_bindee() {
        let ctx = Context::new().extend(ident("B"), tvar("T"));
        let res = stlc::make().infer(&ctx, &let_(binder("x"), broken(), var("B")));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn t_let_error_propagates_body() {
        let ctx = Context::new().extend(ident("A"), tvar("S"));
        let res = stlc::make().infer(&ctx, &let_(binder("x"), var("A"), broken()));
        assert_matches!(res, Err(TypeError::BrokenNode(_)));
    }

    #[test]
    fn t_let_annot() {
        let ctx = Context::new();
        let res = stlc::make().infer(&ctx, &let_(binder_annot("x", tvar("T")), var("A"), var("B")));
        assert_matches!(res, Err(TypeError::NoInferRule(_)));
    }

    #[test]
    fn t_unit() {
        let res = stlc::make().infer(&Context::new(), &unit());
        assert_eq!(res.unwrap(), UNIT);
    }
}
