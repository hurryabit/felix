#![allow(dead_code, unused_variables)]
mod ast;
mod checker;
mod cst;
mod stlc;
mod typ;

pub use checker::{Checker, Context, Result, TypeError, TypeSystem};
pub use typ::Type;
