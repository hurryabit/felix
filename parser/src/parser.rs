use felix_common::{srcloc::Mapper, Problem};
use logos::Logos;

use crate::syntax::{self, NodeKind, TokenKind, TokenKindSet};
use NodeKind::ERROR;
use TokenKind::EOF;

/// Stateful parser for the Rufus language.
pub struct Parser<'a> {
    input: &'a str,
    mapper: &'a Mapper,
    lexer: logos::Lexer<'a, TokenKind>,
    peeked: Option<TokenKind>,
    trivia: Vec<(TokenKind, std::ops::Range<usize>)>,
    open_node_stack: Vec<NodeKind>,
    builder: rowan::GreenNodeBuilder<'a>,
    problems: Vec<Problem>,
}

pub(crate) type Result<T> = std::result::Result<T, Problem>;

pub struct ParseResult {
    pub syntax: syntax::Node,
    pub problems: Vec<Problem>,
}

pub use rowan::Checkpoint;

impl<'a> Parser<'a> {
    /// Create a new parser on the given input.
    pub fn new(input: &'a str, mapper: &'a Mapper) -> Self {
        Self {
            input,
            mapper,
            lexer: TokenKind::lexer(input),
            peeked: None,
            trivia: Vec::new(),
            open_node_stack: Vec::new(),
            builder: rowan::GreenNodeBuilder::new(),
            problems: Vec::new(),
        }
    }

    pub fn run(mut self, rule: fn(&mut Self)) -> ParseResult {
        rule(&mut self);
        assert_eq!(self.peek(), EOF);
        assert!(self.trivia.is_empty());
        let green_node = self.builder.finish();
        ParseResult {
            syntax: rowan::SyntaxNode::new_root(green_node),
            problems: self.problems,
        }
    }

    pub(crate) fn push_problem(&mut self, problem: Problem) {
        self.problems.push(problem);
    }

    pub(crate) fn error(&mut self, message: String) -> Problem {
        let span = self.lexer.span();
        let node = *self.open_node_stack.last().unwrap();
        assert!(node != NodeKind::ERROR);
        let source = format!("parser/{}", node.to_string().to_ascii_lowercase());
        self.mapper
            .error(span.start as u32, span.end as u32, source, message)
    }

    pub(crate) fn commit_trivia(&mut self) {
        for (token, span) in std::mem::take(&mut self.trivia).into_iter() {
            self.builder.token(token.into(), &self.input[span]);
        }
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
                        self.trivia.push((token, self.lexer.span()));
                    } else {
                        break token;
                    }
                }
            }
        };
        self.peeked = Some(token);
        token
    }

    pub(crate) fn expect_error(
        &mut self,
        token: TokenKind,
        expected: impl Into<TokenKindSet>,
    ) -> Problem {
        self.error(format!("Found {}, expected {}.", token, expected.into()))
    }

    pub(crate) fn expect(&mut self, expected: impl Into<TokenKindSet>) -> Result<TokenKind> {
        let expected = expected.into();
        let token = self.peek();
        if token.is(expected) {
            Ok(token)
        } else {
            Err(self.expect_error(token, expected))
        }
    }

    pub(crate) fn consume_any(&mut self) -> TokenKind {
        let token = self.peek();
        if token == TokenKind::EOF {
            panic!("consuming end-of-file");
        }
        self.commit_trivia();
        self.builder
            .token(token.into(), &self.input[self.lexer.span()]);
        self.peeked = None;
        token
    }

    pub(crate) fn consume(&mut self, expected: impl Into<TokenKindSet>) -> Result<TokenKind> {
        self.expect(expected)?;
        Ok(self.consume_any())
    }

    /// Skip tokens until a one from the expected set or EOF is found. This function is
    /// meant for error recovery. Hence, no problems are reported. The skipped
    /// tokens are collected in a single ERROR node. The found token is _not_
    /// consumed but returned.
    pub(crate) fn skip_until(&mut self, expected: impl Into<TokenKindSet>) -> TokenKind {
        let expected = expected.into() | EOF;
        if self.peek().is(expected) {
            return self.peek();
        }
        let mut parser = self.with_node(ERROR);
        while !parser.peek().is(expected) {
            parser.consume_any();
        }
        parser.peek()
    }

    /// Return a checkpoint for usage with `open_node_at` and `with_node_at`.
    pub(crate) fn checkpoint(&mut self) -> Checkpoint {
        self.peek();
        self.commit_trivia();
        self.builder.checkpoint()
    }

    /// Open a new node in the CST without including leading trivia.
    fn open_node(&mut self, node: NodeKind) {
        self.peek();
        self.commit_trivia();
        self.open_root(node);
    }

    /// Retroactively open a new node in the CST without including leading
    /// trivia at the time when the checkpoint was created.
    fn open_node_at(&mut self, checkpoint: Checkpoint, node: NodeKind) {
        self.open_node_stack.push(node);
        self.builder.start_node_at(checkpoint, node.into());
    }

    /// Open a new node in the CST including leading trivia.
    /// This is meant to be used for the root node of the CST. Hence the name.
    fn open_root(&mut self, node: NodeKind) {
        self.open_node_stack.push(node);
        self.builder.start_node(node.into());
    }

    /// Close the current node in the CST without including trailing trivia.
    fn close_node(&mut self) {
        self.builder.finish_node();
        assert!(self.open_node_stack.pop().is_some());
    }

    /// Close the current node in the CST including trivia.
    /// This is meant to be used for the root node of the CST. Hence the name.
    fn close_root(&mut self) {
        self.commit_trivia();
        self.close_node();
    }

    /// Create a sub-parser bracketed with `open_node` and `close_node`.
    pub(crate) fn with_node<'b>(&'b mut self, node: NodeKind) -> Scope<'a, 'b> {
        self.open_node(node);
        Scope::new(self, Self::close_node)
    }

    /// Create a sub-parser bracketed with `open_node_at` and `close_node`.
    pub(crate) fn with_node_at<'b>(
        &'b mut self,
        checkpoint: Checkpoint,
        node: NodeKind,
    ) -> Scope<'a, 'b> {
        self.open_node_at(checkpoint, node);
        Scope::new(self, Self::close_node)
    }

    /// Create a sub-parser bracketed with `open_root` and `close_root`.
    pub(crate) fn with_root<'b>(&'b mut self, root: NodeKind) -> Scope<'a, 'b> {
        self.open_root(root);
        Scope::new(self, Self::close_root)
    }
}

// TODO(MH): Rename into SubParser.
pub struct Scope<'a, 'b> {
    parser: &'b mut Parser<'a>,
    drop_fn: fn(&mut Parser<'a>),
}

impl<'a, 'b> Scope<'a, 'b> {
    fn new(parser: &'b mut Parser<'a>, drop_fn: fn(&mut Parser<'a>)) -> Self {
        Self { parser, drop_fn }
    }
}

impl<'a, 'b> Drop for Scope<'a, 'b> {
    fn drop(&mut self) {
        (self.drop_fn)(self.parser);
    }
}

impl<'a, 'b> std::ops::Deref for Scope<'a, 'b> {
    type Target = Parser<'a>;

    fn deref(&self) -> &Self::Target {
        self.parser
    }
}

impl<'a, 'b> std::ops::DerefMut for Scope<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parser
    }
}
