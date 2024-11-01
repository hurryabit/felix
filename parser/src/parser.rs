use felix_common::{srcloc::Mapper, Problem, SrcSpan};
use logos::Logos;

use crate::syntax::{self, NodeKind, TokenKind, TokenKindSet};
use NodeKind::ERROR;
use TokenKind::EOF;

#[cfg(test)]
static FAKE_INPUT: &str = "0123456789ABCDEF";
// TODO(MH): Use `const fn` instead.
#[cfg(test)]
static FAKE_MAPPER: std::sync::LazyLock<Mapper> =
    std::sync::LazyLock::new(|| Mapper::new(FAKE_INPUT));

/// Stateful parser for the Rufus language.
pub struct Parser<'a> {
    input: &'a str,
    mapper: &'a Mapper,
    lexer:
        Box<dyn Iterator<Item = (std::result::Result<TokenKind, ()>, std::ops::Range<usize>)> + 'a>,
    peeked: Option<(TokenKind, SrcSpan<u32>)>,
    trivia: Vec<(TokenKind, SrcSpan<u32>)>,
    preserve_trivia: bool,
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
            lexer: Box::new(TokenKind::lexer(input).spanned()),
            peeked: None,
            trivia: Vec::new(),
            preserve_trivia: true,
            open_node_stack: Vec::new(),
            builder: rowan::GreenNodeBuilder::new(),
            problems: Vec::new(),
        }
    }

    pub fn without_trivia(mut self) -> Self {
        self.preserve_trivia = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn fake_from_tokens(tokens: impl IntoIterator<Item = TokenKind> + 'a) -> Self {
        Self {
            input: FAKE_INPUT,
            mapper: &FAKE_MAPPER,
            lexer: Box::new(
                tokens
                    .into_iter()
                    .enumerate()
                    .map(|(index, token)| (Ok(token), index..index + 1)),
            ),
            peeked: None,
            trivia: Vec::new(),
            preserve_trivia: true,
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

    #[cfg(test)]
    pub(crate) fn run_partial(
        mut self,
        rule: fn(&mut Self, TokenKindSet) -> Result<()>,
    ) -> ParseResult {
        if let Err(problem) = rule(&mut self, TokenKind::EOF.into()) {
            self.push_problem(problem);
        } else {
            assert_eq!(self.peek(), EOF);
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
        let span = self.peeked.unwrap().1;
        let node = self
            .open_node_stack
            .last()
            .copied()
            .unwrap_or(NodeKind::ERROR);
        let source = format!("parser/{}", node.to_string().to_ascii_lowercase());
        self.mapper.error(span.start, span.end, source, message)
    }

    pub(crate) fn commit_trivia(&mut self) {
        if self.preserve_trivia {
            for (token, span) in self.trivia.drain(..) {
                self.builder
                    .token(token.into(), &self.input[span.into_range()]);
            }
        } else {
            self.trivia.clear();
        }
    }

    fn peek_with_span(&mut self) -> (TokenKind, SrcSpan<u32>) {
        if let Some(token_span) = self.peeked {
            return token_span;
        }
        let token_span = loop {
            match self.lexer.next() {
                None => {
                    let size = self.input.len() as u32;
                    let span = SrcSpan {
                        start: size,
                        end: size,
                    };
                    break (TokenKind::EOF, span);
                }
                Some((token, range)) => {
                    let token = token.unwrap_or(TokenKind::UNKNOWN);
                    let span = SrcSpan::from_range(range);
                    if token.is_trivia() {
                        self.trivia.push((token, span));
                    } else {
                        break (token, span);
                    }
                }
            }
        };
        self.peeked = Some(token_span);
        token_span
    }

    pub(crate) fn peek(&mut self) -> TokenKind {
        self.peek_with_span().0
    }

    pub(crate) fn expecation_error(&mut self, token: TokenKind, expected: TokenKindSet) -> Problem {
        self.error(format!("Found {}, expected {}.", token, expected))
    }

    pub(crate) fn expect(&mut self, expected: TokenKindSet) -> Result<TokenKind> {
        let token = self.peek();
        if token.is(expected) {
            Ok(token)
        } else {
            Err(self.expecation_error(token, expected))
        }
    }

    pub(crate) fn advance(&mut self, expected: impl Into<TokenKindSet>) -> TokenKind {
        let (token, span) = self.peek_with_span();
        assert!(token.is(expected.into()));
        assert!(token != EOF);
        self.commit_trivia();
        self.builder
            .token(token.into(), &self.input[span.into_range()]);
        self.peeked = None;
        token
    }

    pub(crate) fn expect_advance(
        &mut self,
        expected: impl Into<TokenKindSet>,
    ) -> Result<TokenKind> {
        let token = self.expect(expected.into())?;
        Ok(self.advance(token))
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
            let token = parser.peek();
            parser.advance(token);
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
