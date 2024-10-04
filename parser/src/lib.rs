mod kind;
mod parser;
pub mod rules;

pub use kind::{dump_syntax, SyntaxElement, SyntaxNode, SyntaxToken};
pub use parser::{ParseError, ParseResult, Parser};

#[cfg(test)]
mod tests;
