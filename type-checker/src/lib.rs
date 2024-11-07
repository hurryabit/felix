#![allow(dead_code)]
mod checker;
mod ast;
pub mod stlc;
mod r#type;

pub use checker::{Checker, Context, Result, TypeError, TypeSystem};
pub use r#type::Type;
