// This module implements a parser for the grammar provided in notes.md.
use crate::first::{AliasKind, First};
use crate::parser::{Parser, Result};
use crate::syntax::{NodeKind, TokenKind, TokenKindSet, INFIX_OPS, LITERALS, PREFIX_OPS};

use AliasKind::*;
use NodeKind::*;
use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn program(&mut self) {
        let first = DEFN.first() | EOF;
        let mut parser = self.with_root(PROGRAM);
        while parser.peek() != EOF {
            if let Err(problem) = parser.defn(DEFN.first()) {
                parser.push_problem(problem);
                parser.skip_until(first);
            }
        }
    }

    pub(crate) fn defn(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        match self.peek() {
            token if token.starts(DEFN_FN) => self.defn_fn(follow),
            token => Err(self.expecation_error(token, DEFN.first())),
        }
    }

    pub(crate) fn defn_fn(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(DEFN_FN);
        parser.expect_advance(KW_FN)?;
        parser.expect_advance(IDENT)?;
        parser.params_fn(BLOCK.first())?;
        parser.block(follow)
    }

    pub(crate) fn block(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(BLOCK);
        parser.expect_advance(LBRACE)?;
        parser.block_inner(RBRACE)?;
        parser.expect_advance(RBRACE)?;
        Ok(())
    }

    pub(crate) fn block_inner(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let parser = self;
        loop {
            match parser.peek() {
                token if token.starts(STMT_LET) => parser.stmt_let(BLOCK_INNER.first() | follow)?,
                token if token.starts(STMT_IF) => parser.stmt_if(BLOCK_INNER.first() | follow)?,
                token if token.starts(EXPR) => {
                    let checkpoint = parser.checkpoint();
                    parser.expr(EQUALS | SEMI | follow)?;
                    match parser.peek() {
                        EQUALS => {
                            parser.advance();
                            let mut parser = parser.with_node_at(checkpoint, STMT_ASSIGN);
                            parser.expr(SEMI)?;
                            parser.expect_advance(SEMI)?;
                        }
                        SEMI => {
                            parser.advance();
                            parser.with_node_at(checkpoint, STMT_EXPR);
                        }
                        token if token.is(follow) => return Ok(()),
                        token => return Err(parser.expecation_error(token, EQUALS | SEMI | follow)),
                    }
                }
                token if token.is(follow) => return Ok(()),
                token => return Err(parser.expecation_error(token, BLOCK_INNER.first())),
            }
        }
    }

    pub(crate) fn stmt_if(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let mut parser = self.with_node(STMT_IF);
        parser.expect_advance(KW_IF)?;
        parser.expr(BLOCK.first())?;
        parser.block(KW_ELSE | follow)?;
        // TODO(MH): We get a better error message if we know the follow set.
        if parser.peek() != KW_ELSE {
            return Ok(());
        }
        parser.advance();
        match parser.peek() {
            // TODO(MH): Turn the tail recursion into a loop?
            token if token.starts(STMT_IF) => parser.stmt_if(follow),
            token if token.starts(BLOCK) => parser.block(follow),
            token => Err(parser.expecation_error(token, STMT_IF.first() | BLOCK.first())),
        }
    }

    pub(crate) fn stmt_let(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(STMT_LET);
        parser.expect_advance(KW_LET)?;
        match parser.peek() {
            KW_REC => {
                parser.advance();
            }
            token if token.starts(BINDER) => {}
            token => return Err(parser.expecation_error(token, KW_REC | BINDER.first())),
        }
        parser.binder(EQUALS)?;
        parser.expect_advance(EQUALS)?;
        parser.expr(SEMI)?;
        parser.expect_advance(SEMI)?;
        Ok(())
    }

    pub(crate) fn expr(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        match self.peek() {
            token if token.starts(EXPR_CLOSURE) => self.expr_closure(follow),
            token if token.starts(LEVEL_TERTIARY) => self.level_tertiary(follow),
            token => Err(self.expecation_error(token, EXPR.first())),
        }
    }

    pub(crate) fn expr_closure(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(EXPR_CLOSURE);
        parser.params_closure(EXPR.first())?;
        parser.expr(follow)
    }

    pub(crate) fn level_tertiary(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let checkpoint = self.checkpoint();
        self.level_infix(follow | QUERY)?;
        if self.expect(follow | QUERY)? != QUERY {
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, EXPR_TERTIARY);
        parser.advance();
        parser.level_tertiary(COLON)?;
        parser.expect_advance(COLON)?;
        parser.level_tertiary(follow)
    }

    // NOTE(MH): We use Pratt parsing to resolve precendence. We use matklad's
    // trick of different binding powers on the left and right to resolve
    // associativity. See
    // https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
    pub(crate) fn level_infix(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        fn binding_power(op: TokenKind) -> (u32, u32) {
            match op {
                BAR_BAR => (15, 10),
                AMPER_AMPER => (25, 20),
                EQUALS_EQUALS | BANG_EQUALS | LANGLE | LANGLE_EQUALS | RANGLE | RANGLE_EQUALS => {
                    (30, 30)
                }
                PLUS | MINUS => (40, 45),
                STAR | SLASH | PERCENT => (50, 55),
                token => unreachable!("token is not infix operator: {:?}", token),
            }
        }

        #[derive(Clone, Copy)]
        struct StackEntry {
            checkpoint: rowan::Checkpoint,
            op: TokenKind,
            right_power: u32,
        }

        let follow = follow.into();
        let mut stack: Vec<StackEntry> = Vec::new();
        let mut checkpoint = self.checkpoint();
        self.level_prefix(INFIX_OPS | LEVEL_PREFIX.first())?;

        let res = loop {
            let op = self.peek();
            if !op.is(INFIX_OPS) {
                break Ok(());
            }
            let (left_power, right_power) = binding_power(op);
            let op_node = loop {
                if let Some(top) = stack.last().copied() {
                    if top.right_power >= left_power {
                        checkpoint = top.checkpoint;
                        self.with_node_at(top.checkpoint, EXPR_INFIX);
                        stack.pop();
                        if top.right_power > left_power {
                            continue;
                        }
                        let problem = self.error(format!(
                            "Cannot chain comparison operators {} and {}",
                            top.op, op
                        ));
                        self.push_problem(problem);
                        break ERROR;
                    }
                }
                break OP_INFIX;
            };
            assert!(self.with_node(op_node).expect_advance(op).is_ok());
            stack.push(StackEntry {
                checkpoint,
                op,
                right_power,
            });
            checkpoint = self.checkpoint();
            if let Err(problem) = self.level_prefix(follow) {
                break Err(problem);
            }
        };
        for entry in stack.into_iter().rev() {
            self.with_node_at(entry.checkpoint, EXPR_INFIX);
        }
        res
    }

    pub(crate) fn level_prefix(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut stack: Vec<rowan::Checkpoint> = Vec::new();
        let res = loop {
            let token = self.peek();
            if token.is(PREFIX_OPS) {
                stack.push(self.checkpoint());
                self.with_node(OP_PREFIX).advance();
            } else if token.starts(LEVEL_POSTFIX) {
                break self.level_postfix(follow);
            } else {
                break Err(self.expecation_error(token, LEVEL_PREFIX.first()));
            }
        };
        for checkpoint in stack.into_iter().rev() {
            self.with_node_at(checkpoint, EXPR_PREFIX);
        }
        res
    }

    pub(crate) fn level_postfix(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let checkpoint = self.checkpoint();
        self.level_atom(LPAREN | DOT | follow)?;
        while self.peek().is(DOT | ARGS.first()) {
            match self.peek() {
                DOT => {
                    let mut parser = self.with_node_at(checkpoint, EXPR_SELECT);
                    parser.advance();
                    parser.expect_advance(LIT_NAT)?;
                }
                token if token.starts(ARGS) => self.with_node_at(checkpoint, EXPR_CALL).args(LPAREN | DOT | follow)?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    pub(crate) fn level_atom(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        match self.peek() {
            IDENT => {
                self.with_node(EXPR_VAR).expect_advance(IDENT)?;
                Ok(())
            }
            LPAREN => self.expr_paren_or_tuple(follow),
            token if token.starts(BLOCK) => self.block(follow),
            token if token.is(LITERALS) => {
                self.with_node(EXPR_LIT).expect_advance(LITERALS)?;
                Ok(())
            }
            token => Err(self.expecation_error(token, LEVEL_ATOM.first())),
        }
    }

    fn expr_paren_or_tuple(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(LPAREN)?;
        if self.expect(RPAREN | EXPR.first())? == RPAREN {
            self.with_node_at(checkpoint, EXPR_TUPLE).advance();
            return Ok(());
        }
        self.expr(RPAREN | COMMA)?;
        if self.expect(RPAREN | COMMA)? == RPAREN {
            self.with_node_at(checkpoint, EXPR_PAREN).advance();
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, EXPR_TUPLE);
        parser.expect_advance(COMMA)?;
        if parser.expect(RPAREN | EXPR.first())? == RPAREN {
            parser.advance();
            return Ok(());
        }
        loop {
            parser.expr(COMMA | RPAREN)?;
            if parser.expect_advance(COMMA | RPAREN)? == RPAREN {
                return Ok(());
            }
        }
    }

    fn params(&mut self, ldelim: TokenKind, rdelim: TokenKind) -> Result<()> {
        self.expect_advance(ldelim)?;
        if self.peek() == rdelim {
            self.expect_advance(rdelim)?;
            return Ok(());
        }
        loop {
            self.binder(COMMA | rdelim)?;
            if self.expect_advance(COMMA | rdelim)? == rdelim {
                return Ok(());
            }
        }
    }

    pub(crate) fn params_fn(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        self.with_node(PARAMS_FN).params(LPAREN, RPAREN)
    }

    pub(crate) fn params_closure(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(PARAMS_CLOSURE);
        match parser.peek() {
            BAR => parser.params(BAR, BAR),
            BAR_BAR => {
                parser.advance();
                Ok(())
            }
            token => Err(parser.expecation_error(token, PARAMS_CLOSURE.first())),
        }
    }

    pub(crate) fn binder(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(BINDER);
        if parser.expect_advance(KW_MUT | IDENT)? == KW_MUT {
            parser.expect_advance(IDENT)?;
        }
        Ok(())
    }

    pub(crate) fn args(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(ARGS);
        parser.expect_advance(LPAREN)?;
        if parser.peek() == RPAREN {
            parser.expect_advance(RPAREN)?;
            return Ok(());
        }
        loop {
            parser.expr(COMMA | RPAREN)?;
            if parser.expect_advance(COMMA | RPAREN)? == RPAREN {
                return Ok(());
            }
        }
    }
}
