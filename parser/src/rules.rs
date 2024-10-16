// This module implements a parser for the grammar provided in notes.md.
use crate::first::First;
use crate::parser::{Parser, Result};
use crate::syntax::{
    AliasKind, NodeKind, TokenKind, TokenKindSet, INFIX_OPS, LITERALS, PREFIX_OPS,
};

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
            token if token.starts(DEFN_TYPE) => self.defn_type(follow),
            token if token.starts(DEFN_FN) => self.defn_fn(follow),
            token => Err(self.expecation_error(token, DEFN.first())),
        }
    }

    pub(crate) fn defn_type(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(DEFN_TYPE);
        parser.expect_advance(KW_TYPE)?;
        parser.expect_advance(IDENT)?;
        parser.expect_advance(EQUALS)?;
        parser.type_(SEMI)?;
        parser.expect_advance(SEMI)?;
        Ok(())
    }

    pub(crate) fn type_(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        self.level_type_fn(follow)
    }

    pub(crate) fn level_type_fn(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        if self.expect(LEVEL_UNION.first() | KW_FN)? == KW_FN {
            // TODO(MH): Turn this indirect tail recursion into a loop.
            self.type_fn(follow)
        } else {
            self.level_union(follow)
        }
    }

    pub(crate) fn type_fn(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(TYPE_FN);
        parser.expect_advance(KW_FN)?;
        parser.expect_advance(LPAREN)?;
        parser.list_types(RPAREN)?;
        parser.expect_advance(RPAREN)?;
        parser.expect_advance(MINUS_RANGLE)?;
        parser.level_type_fn(follow)
    }

    pub(crate) fn level_union(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let checkpoint = self.checkpoint();
        self.level_intersection(follow | BAR)?;
        if self.expect(follow | BAR)? != BAR {
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, TYPE_UNION);
        parser.advance();
        // TODO(MH): Turn tail recursion into loop.
        parser.level_union(follow)
    }

    pub(crate) fn level_intersection(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let checkpoint = self.checkpoint();
        self.level_complement(follow | BAR)?;
        if self.expect(follow | AMPER)? != AMPER {
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, TYPE_INTERSECTION);
        parser.advance();
        // TODO(MH): Turn tail recursion into loop.
        parser.level_intersection(follow)
    }

    pub(crate) fn level_complement(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        if self.expect(LEVEL_BASIC.first() | TILDE)? == TILDE {
            let mut parser = self.with_node(TYPE_COMPLEMENT);
            parser.advance();
            // TODO(MH): Turn tail recursion into loop.
            parser.level_complement(follow)
        } else {
            self.level_basic(follow)
        }
    }

    pub(crate) fn level_basic(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        match self.peek() {
            LPAREN => self.type_paren_or_tuple(follow),
            token if token.starts(TYPE_BUILTIN) => {
                self.with_node(TYPE_BUILTIN).advance();
                Ok(())
            }
            token if token.starts(TYPE_REF) => {
                self.with_node(TYPE_REF).advance();
                Ok(())
            }
            token => return Err(self.expecation_error(token, LEVEL_BASIC.first())),
        }
    }

    pub(crate) fn type_paren_or_tuple(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(LPAREN)?;
        if self.expect(RPAREN | TYPE.first())? == RPAREN {
            self.with_node_at(checkpoint, TYPE_TUPLE).advance();
            return Ok(());
        }
        self.type_(RPAREN | COMMA)?;
        if self.expect(RPAREN | COMMA)? == RPAREN {
            self.with_node_at(checkpoint, TYPE_PAREN).advance();
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, TYPE_TUPLE);
        parser.expect_advance(COMMA)?;
        if parser.expect(RPAREN | TYPE.first())? == RPAREN {
            parser.advance();
            return Ok(());
        }
        loop {
            parser.type_(COMMA | RPAREN)?;
            if parser.expect_advance(COMMA | RPAREN)? == RPAREN {
                return Ok(());
            }
        }
    }

    pub(crate) fn list_types(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let follow = follow.into();
        let mut parser = self.with_node(LIST_TYPES);
        if !parser.expect(TYPE.first() | follow)?.starts(TYPE) {
            return Ok(());
        }
        parser.type_(COMMA | follow)?;
        while parser.expect(COMMA | follow)? == COMMA {
            parser.advance();
            parser.type_(COMMA | follow)?;
        }
        Ok(())
    }

    pub(crate) fn defn_fn(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(DEFN_FN);
        parser.expect_advance(KW_FN)?;
        parser.expect_advance(IDENT)?;
        parser.params(BLOCK.first())?;
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
                KW_LET => parser.stmt_lets(BLOCK_INNER.first() | follow)?,
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

    pub(crate) fn stmt_lets(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(KW_LET)?;
        if self.expect(KW_REC | BINDER.first())? != KW_REC {
            let mut parser = self.with_node_at(checkpoint, STMT_LET);
            parser.binding(SEMI)?;
            parser.expect_advance(SEMI)?;
        } else {
            let mut parser = self.with_node_at(checkpoint, STMT_LET_REC);
            parser.advance();
            loop {
                parser.binding(KW_AND | SEMI)?;
                if parser.expect_advance(KW_AND | SEMI)? == SEMI {
                    break;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn expr(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        self.level_tertiary(follow)
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
                token if token.starts(ARGS) => self
                    .with_node_at(checkpoint, EXPR_CALL)
                    .args(LPAREN | DOT | follow)?,
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
            token if token.is(LITERALS) => {
                self.with_node(EXPR_LIT).expect_advance(LITERALS)?;
                Ok(())
            }
            token if token.starts(EXPR_FN) => self.expr_fn(follow),
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

    pub(crate) fn expr_fn(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(EXPR_FN);
        parser.expect_advance(KW_FN)?;
        parser.params(BLOCK.first())?;
        parser.block(follow)
    }

    pub(crate) fn params(&mut self, _follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(PARAMS);
        parser.expect_advance(LPAREN)?;
        if parser.peek() == RPAREN {
            parser.advance();
            return Ok(());
        }
        loop {
            parser.binder(COMMA | RPAREN)?;
            if parser.expect_advance(COMMA | RPAREN)? == RPAREN {
                return Ok(());
            }
        }
    }

    pub(crate) fn binding(&mut self, follow: impl Into<TokenKindSet>) -> Result<()> {
        let mut parser = self.with_node(BINDING);
        parser.binder(EQUALS)?;
        parser.expect_advance(EQUALS)?;
        parser.expr(follow)
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
