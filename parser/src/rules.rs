// This module implements a parser for the grammar provided in notes.md.
use crate::first::{AliasKind, First};
use crate::parser::{Parser, Result};
use crate::syntax::{NodeKind, TokenKind, INFIX_OPS, LITERALS, PREFIX_OPS};

use AliasKind::*;
use NodeKind::*;
use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn program(&mut self) {
        let first = DEFN.first() | EOF;
        let mut parser = self.with_root(PROGRAM);
        while parser.peek() != EOF {
            if let Err(problem) = parser.defn() {
                parser.push_problem(problem);
                parser.skip_until(first);
            }
        }
    }

    fn defn(&mut self) -> Result<()> {
        match self.peek() {
            KW_FN => self.defn_fn(),
            token => Err(self.expecation_error(token, DEFN.first())),
        }
    }

    fn defn_fn(&mut self) -> Result<()> {
        let mut parser = self.with_node(DEFN_FN);
        parser.expect_advance(KW_FN)?;
        parser.expect_advance(IDENT)?;
        parser.params_fn()?;
        parser.expr_block()
    }

    fn expr_block(&mut self) -> Result<()> {
        let mut parser = self.with_node(EXPR_BLOCK);
        parser.expect_advance(LBRACE)?;
        loop {
            match parser.peek() {
                RBRACE => {
                    parser.advance();
                    return Ok(());
                }
                KW_LET => parser.stmt_let()?,
                KW_IF => parser.stmt_if()?,
                token if token.starts(EXPR) => {
                    let checkpoint = parser.checkpoint();
                    parser.expr()?;
                    match parser.peek() {
                        EQUALS => {
                            parser.advance();
                            let mut parser = parser.with_node_at(checkpoint, STMT_ASSIGN);
                            parser.expr()?;
                            parser.expect_advance(SEMI)?;
                        }
                        SEMI => {
                            parser.advance();
                            parser.with_node_at(checkpoint, STMT_EXPR);
                        }
                        RBRACE => {
                            parser.advance();
                            return Ok(());
                        }
                        token => return Err(parser.expecation_error(token, EQUALS | SEMI | RBRACE)),
                    }
                }
                token => {
                    return Err(parser.expecation_error(token, RBRACE | STMT.first() | EXPR.first()))
                }
            }
        }
    }

    fn stmt_if(&mut self) -> Result<()> {
        let mut parser = self.with_node(STMT_IF);
        parser.expect_advance(KW_IF)?;
        parser.expr()?;
        parser.expr_block()?;
        // TODO(MH): We get a better error message if we know the follow set.
        if parser.peek() != KW_ELSE {
            return Ok(());
        }
        parser.advance();
        match parser.peek() {
            // TODO(MH): Turn the tail recursion into a loop?
            KW_IF => parser.stmt_if(),
            LBRACE => parser.expr_block(),
            token => Err(parser.expecation_error(token, STMT_IF.first() | EXPR_BLOCK.first())),
        }
    }

    fn stmt_let(&mut self) -> Result<()> {
        let mut parser = self.with_node(STMT_LET);
        parser.expect_advance(KW_LET)?;
        match parser.peek() {
            KW_REC => {
                parser.advance();
            }
            token if token.starts(BINDER) => {}
            token => return Err(parser.expecation_error(token, KW_REC | BINDER.first())),
        }
        parser.binder()?;
        parser.expect_advance(EQUALS)?;
        parser.expr()?;
        parser.expect_advance(SEMI)?;
        Ok(())
    }

    pub(crate) fn expr(&mut self) -> Result<()> {
        match self.peek() {
            BAR => self.expr_closure(),
            KW_IF => self.expr_if(),
            token if token.starts(LEVEL_INFIX) => self.level_infix(),
            token => Err(self.expecation_error(token, EXPR.first())),
        }
    }

    fn expr_closure(&mut self) -> Result<()> {
        let mut parser = self.with_node(EXPR_CLOSURE);
        parser.params_closure()?;
        parser.expr()
    }

    fn expr_if(&mut self) -> Result<()> {
        let mut parser = self.with_node(EXPR_IF);
        parser.expect_advance(KW_IF)?;
        parser.expr()?;
        parser.expr_block()?;
        parser.expect_advance(KW_ELSE)?;
        match parser.peek() {
            KW_IF => parser.expr_if(),
            LBRACE => parser.expr_block(),
            token => Err(parser.expecation_error(token, KW_IF | LBRACE)),
        }
    }

    // NOTE(MH): We use Pratt parsing to resolve precendence. We use matklad's
    // trick of different binding powers on the left and right to resolve
    // associativity. See
    // https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
    fn level_infix(&mut self) -> Result<()> {
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

        let mut stack: Vec<StackEntry> = Vec::new();
        let mut checkpoint = self.checkpoint();
        self.level_prefix()?;

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
            if let Err(problem) = self.level_prefix() {
                break Err(problem);
            }
        };
        for entry in stack.into_iter().rev() {
            self.with_node_at(entry.checkpoint, EXPR_INFIX);
        }
        res
    }

    fn level_prefix(&mut self) -> Result<()> {
        let mut stack: Vec<rowan::Checkpoint> = Vec::new();
        let res = loop {
            let token = self.peek();
            if token.is(PREFIX_OPS) {
                stack.push(self.checkpoint());
                self.with_node(OP_PREFIX).advance();
            } else if token.starts(LEVEL_POSTFIX) {
                break self.level_postfix();
            } else {
                break Err(self.expecation_error(token, LEVEL_PREFIX.first()));
            }
        };
        for checkpoint in stack.into_iter().rev() {
            self.with_node_at(checkpoint, EXPR_PREFIX);
        }
        res
    }

    fn level_postfix(&mut self) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.level_atom()?;
        while self.peek().is(LPAREN | DOT) {
            match self.peek() {
                LPAREN => self.with_node_at(checkpoint, EXPR_CALL).args()?,
                DOT => {
                    let mut parser = self.with_node_at(checkpoint, EXPR_SELECT);
                    parser.expect_advance(DOT)?;
                    parser.expect_advance(LIT_NAT)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn level_atom(&mut self) -> Result<()> {
        match self.peek() {
            IDENT => {
                self.with_node(EXPR_VAR).expect_advance(IDENT)?;
                Ok(())
            }
            LBRACE => self.expr_block(),
            LPAREN => self.expr_paren_or_tuple(),
            token if token.is(LITERALS) => {
                self.with_node(EXPR_LIT).expect_advance(LITERALS)?;
                Ok(())
            }
            token => Err(self.expecation_error(token, LEVEL_ATOM.first())),
        }
    }

    fn expr_paren_or_tuple(&mut self) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(LPAREN)?;
        if self.expect(RPAREN | EXPR.first())? == RPAREN {
            self.with_node_at(checkpoint, EXPR_TUPLE).advance();
            return Ok(());
        }
        self.expr()?;
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
            parser.expr()?;
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
            self.binder()?;
            if self.expect_advance(COMMA | rdelim)? == rdelim {
                return Ok(());
            }
        }
    }

    fn params_fn(&mut self) -> Result<()> {
        self.with_node(PARAMS_FN).params(LPAREN, RPAREN)
    }

    fn params_closure(&mut self) -> Result<()> {
        self.with_node(PARAMS_CLOSURE).params(BAR, BAR)
    }

    fn binder(&mut self) -> Result<()> {
        let mut parser = self.with_node(BINDER);
        if parser.expect_advance(KW_MUT | IDENT)? == KW_MUT {
            parser.expect_advance(IDENT)?;
        }
        Ok(())
    }

    fn args(&mut self) -> Result<()> {
        let mut parser = self.with_node(ARGS);
        parser.expect_advance(LPAREN)?;
        if parser.peek() == RPAREN {
            parser.expect_advance(RPAREN)?;
            return Ok(());
        }
        loop {
            parser.expr()?;
            if parser.expect_advance(COMMA | RPAREN)? == RPAREN {
                return Ok(());
            }
        }
    }
}

// Idea for testing first sets:
// 1. For every token T in X.first(), check that parse(X) fails at the
//    Unknown in the token sequence [T, Unknown].
// 2. For every token T not in X.first(), check that parse(X) fails at
//    the T in the token sequence [T].
// The spans in problems should help identify where a parser failed.
// For this to make sense, the implementation of parse(X) should not
// by calling self.expect(X.first()) but rather check for what it actually
// needs.
// This idea can be extended for nodes like EXPR_BLOCK where the second
// token is more interesting.
