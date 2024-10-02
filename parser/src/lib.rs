mod kind;
mod parser;
pub mod rules;

pub use kind::{SyntaxElement, SyntaxNode, SyntaxToken, dump_syntax};
pub use parser::{ParseError, ParseResult, Parser};

#[cfg(test)]
mod tests;
