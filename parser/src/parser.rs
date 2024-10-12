use felix_common::{srcloc::Mapper, Problem};
use logos::Logos;

use crate::syntax::{self, NodeKind, TokenKind, TokenKindSet};

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
        let green_node = self.builder.finish();
        ParseResult {
            syntax: rowan::SyntaxNode::new_root(green_node),
            problems: self.problems,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_pseudo(mut self, pseudo: crate::first::PseudoKind) -> ParseResult {
        let result = || -> Result<_> {
            let mut parser = self.with_node(NodeKind::PROGRAM);
            parser.parse_pseudo(pseudo)?;
            parser.expect(TokenKind::EOF)
        }();
        if let Err(problem) = result {
            self.push_problem(problem);
        }
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

    pub(crate) fn expect(&mut self, expected: impl Into<TokenKindSet>) -> Result<TokenKind> {
        let expected = expected.into();
        let token = self.peek();
        if token.is(expected) {
            Ok(token)
        } else {
            Err(self.error(format!("Found {}, expected {}.", token, expected)))
        }
    }

    pub(crate) fn consume(&mut self, expected: impl Into<TokenKindSet>) -> Result<TokenKind> {
        let expected = expected.into();
        let token = self.expect(expected)?;
        if token == TokenKind::EOF {
            panic!("consume end-of-file");
        }
        self.commit_trivia();
        self.builder
            .token(token.into(), &self.input[self.lexer.span()]);
        self.peeked = None;
        Ok(token)
    }

    pub(crate) fn with_immediate_node<'b>(&'b mut self, node: NodeKind) -> Scope<'a, 'b> {
        self.open_node_stack.push(node);
        self.builder.start_node(node.into());
        Scope::new(self, Self::close_node)
    }

    pub(crate) fn with_node<'b>(&'b mut self, node: NodeKind) -> Scope<'a, 'b> {
        self.peek();
        self.commit_trivia();
        self.with_immediate_node(node)
    }

    pub(crate) fn checkpoint(&mut self) -> rowan::Checkpoint {
        self.peek();
        self.commit_trivia();
        self.builder.checkpoint()
    }

    pub(crate) fn with_node_at<'b>(
        &'b mut self,
        checkpoint: rowan::Checkpoint,
        node: NodeKind,
    ) -> Scope<'a, 'b> {
        self.open_node_stack.push(node);
        self.builder.start_node_at(checkpoint, node.into());
        Scope::new(self, Self::close_node)
    }

    fn close_node(parser: &mut Parser) {
        parser.builder.finish_node();
        assert!(parser.open_node_stack.pop().is_some());
    }
}

pub struct Scope<'a, 'b> {
    parser: &'b mut Parser<'a>,
    drop_fn: fn(&mut Parser),
}

impl<'a, 'b> Scope<'a, 'b> {
    fn new(parser: &'b mut Parser<'a>, drop_fn: fn(&mut Parser)) -> Self {
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
