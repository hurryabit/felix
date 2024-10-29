#![allow(dead_code)]
mod checker;
mod ast;
pub mod stlc;
mod typ;

pub use checker::{Checker, Context, Result, TypeError, TypeSystem};
pub use typ::Type;
