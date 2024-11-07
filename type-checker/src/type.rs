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
