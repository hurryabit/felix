use logos::Logos;
use felix_common::{Problem, srcloc::Mapper};

use crate::syntax::{self, NodeKind, TokenKind, TokenKindSet};

/// Stateful parser for the Rufus language.
pub struct Parser<'a> {
    input: &'a str,
    mapper: &'a Mapper,
    lexer: logos::Lexer<'a, TokenKind>,
    peeked: Option<TokenKind>,
    open_node_stack: Vec<NodeKind>,
    builder: rowan::GreenNodeBuilder<'a>,
    problems: Vec<Problem>,
}

pub(crate) type Result<T> = std::result::Result<T, Problem>;

pub struct ParseResult {
    pub syntax: syntax::Node,
    pub problems: Vec<Problem>,
}

impl<'a> Parser<'a> {
    /// Create a new parser on the given input.
    pub fn new(input: &'a str, mapper: &'a Mapper) -> Self {
        Self {
            input,
            mapper,
            lexer: TokenKind::lexer(input),
            peeked: None,
            open_node_stack: Vec::new(),
            builder: rowan::GreenNodeBuilder::new(),
            problems: Vec::new(),
        }
    }

    pub fn run(mut self, rule: fn(&mut Self)) -> ParseResult {
        rule(&mut self);
        let green_node = self.builder.finish();
        ParseResult {
            syntax: rowan::SyntaxNode::new_root(green_node),
            problems: self.problems,
        }
    }

    pub(crate) fn checkpoint(&mut self) -> rowan::Checkpoint {
        self.peek(); // Put whitespace before the checkpoint.
        self.builder.checkpoint()
    }

    pub(crate) fn push_problem(&mut self, problem: Problem) {
        self.problems.push(problem);
    }

    pub(crate) fn error(&mut self, message: String) -> Problem {
        let span = self.lexer.span();
        let node = *self.open_node_stack.last().unwrap();
        assert!(node != NodeKind::ERROR);
        let source = format!("parser/{}", node.to_string().to_ascii_lowercase());
        self.mapper.error(span.start as u32, span.end as u32, source, message)
    }

    fn unexpected_token_error(&mut self, expected: TokenKindSet) -> Problem {
        let found = self.peek();
        self.error(format!("Found {}, expected {}.", found, expected))
    }

    pub(crate) fn peek(&mut self) -> TokenKind {
        if let Some(token) = self.peeked {
            return token;
        }
        let token = loop {
            match self.lexer.next() {
                None => break TokenKind::EOF,
                Some(Err(_)) => break TokenKind::UNKNOWN,
                Some(Ok(token)) => {
                    if token.is_trivia() {
                        self.builder
                            .token(token.into(), &self.input[self.lexer.span()]);
                    } else {
                        break token;
                    }
                }
            }
        };
        self.peeked = Some(token);
        token
    }

    pub(crate) fn expect(&mut self, expected: impl Into<TokenKindSet>) -> Result<TokenKind> {
        let expected = expected.into();
        let token = self.peek();
        if token.is(expected) {
            Ok(token)
        } else {
            Err(self.unexpected_token_error(expected))
        }
    }

    pub(crate) fn consume(&mut self, expected: impl Into<TokenKindSet>) -> Result<TokenKind> {
        let expected = expected.into();
        let token = self.expect(expected)?;
        if token == TokenKind::EOF {
            panic!("consume end-of-file");
        }
        self.builder
            .token(token.into(), &self.input[self.lexer.span()]);
        self.peeked = None;
        Ok(token)
    }

    pub(crate) fn with_node<'b>(&'b mut self, node: NodeKind) -> NodeScope<'a, 'b> {
        NodeScope::new(self, node)
    }

    pub(crate) fn with_node_at<'b>(
        &'b mut self,
        checkpoint: rowan::Checkpoint,
        node: NodeKind,
    ) -> NodeScope<'a, 'b> {
        NodeScope::new_at_checkpoint(self, checkpoint, node)
    }
}

pub struct NodeScope<'a, 'b> {
    parser: &'b mut Parser<'a>,
}

impl<'a, 'b> NodeScope<'a, 'b> {
    fn new(parser: &'b mut Parser<'a>, node: NodeKind) -> Self {
        parser.open_node_stack.push(node);
        parser.builder.start_node(node.into());
        Self { parser }
    }

    fn new_at_checkpoint(
        parser: &'b mut Parser<'a>,
        checkpoint: rowan::Checkpoint,
        node: NodeKind,
    ) -> Self {
        parser.open_node_stack.push(node);
        parser.builder.start_node_at(checkpoint, node.into());
        Self { parser }
    }
}

impl<'a, 'b> Drop for NodeScope<'a, 'b> {
    fn drop(&mut self) {
        self.parser.builder.finish_node();
        assert!(self.parser.open_node_stack.pop().is_some());
    }
}

impl<'a, 'b> std::ops::Deref for NodeScope<'a, 'b> {
    type Target = Parser<'a>;

    fn deref(&self) -> &Self::Target {
        self.parser
    }
}

impl<'a, 'b> std::ops::DerefMut for NodeScope<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parser
    }
}
