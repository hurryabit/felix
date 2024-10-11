// This module implements a parser for the grammar provided in notes.md.
use crate::first::{First, PseudoKind};
use crate::parser::{Parser, Result};
use crate::syntax::{NodeKind, TokenKind, INFIX_OPS, LITERALS, PREFIX_OPS};

use NodeKind::*;
use PseudoKind::*;
use TokenKind::*;

impl<'a> Parser<'a> {
    fn parse(&mut self, node: NodeKind) -> Result<()> {
        self.expect(node.first())?;
        let mut parser = self.with_node(node);
        match node {
            PROGRAM => unreachable!(),
            DEFN_FN => parser.defn_fn(),
            EXPR_BLOCK => parser.expr_block(),
            STMT_ASSIGN => unreachable!(),
            STMT_EXPR => unreachable!(),
            STMT_IF => parser.stmt_if(),
            STMT_LET => parser.stmt_let(),
            EXPR_CLOSURE => parser.expr_closure(),
            EXPR_IF => parser.expr_if(),
            EXPR_INFIX => unreachable!(),
            EXPR_PREFIX => unreachable!(),
            EXPR_CALL => unreachable!(),
            EXPR_SELECT => unreachable!(),
            EXPR_VAR => unreachable!(),
            EXPR_LIT => unreachable!(),
            EXPR_TUPLE => unreachable!(),
            EXPR_PAREN => unreachable!(),
            PARAMS_CLOSURE => parser.params(BAR, BAR),
            PARAMS_FN => parser.params(LPAREN, RPAREN),
            BINDER => parser.binder(),
            ARGS => parser.args(),
            OP_INFIX => unreachable!(),
            OP_PREFIX => unreachable!(),
            ERROR => unreachable!(),
        }
    }

    fn parse_pseudo(&mut self, pseudo: PseudoKind) -> Result<()> {
        self.expect(pseudo.first())?;
        match pseudo {
            DEFN => self.defn(),
            STMT => unreachable!(),
            EXPR => self.expr(),
            LEVEL_INFIX => self.level_infix(),
            LEVEL_PREFIX => self.level_prefix(),
            LEVEL_POSTFIX => self.level_postfix(),
            LEVEL_ATOM => self.level_atom(),
        }
    }

    pub fn program(&mut self) {
        let first = PROGRAM.first() | EOF;
        let mut parser = self.with_node(PROGRAM);
        loop {
            match parser.expect(first) {
                Err(err) => {
                    parser.push_error(err);
                    if parser.peek().is(!first) {
                        let mut parser = parser.with_node(ERROR);
                        while parser.peek().is(!first) {
                            let _ = parser.consume(!first);
                        }
                    }
                }
                Ok(EOF) => return,
                Ok(_) => {
                    if let Err(err) = parser.parse_pseudo(DEFN) {
                        parser.push_error(err);
                        if parser.peek().is(!first) {
                            let mut parser = parser.with_node(ERROR);
                            while parser.peek().is(!first) {
                                let _ = parser.consume(!first);
                            }
                        }
                    }
                }
            }
        }
    }

    fn defn(&mut self) -> Result<()> {
        match self.peek() {
            KW_FN => self.parse(DEFN_FN),
            _ => unreachable!(),
        }
    }

    fn defn_fn(&mut self) -> Result<()> {
        self.consume(KW_FN)?;
        self.consume(IDENT)?;
        self.parse(PARAMS_FN)?;
        self.parse(EXPR_BLOCK)
    }

    fn expr_block(&mut self) -> Result<()> {
        self.consume(LBRACE)?;
        loop {
            match self.expect(RBRACE | STMT.first() | EXPR.first())? {
                RBRACE => {
                    self.consume(RBRACE)?;
                    return Ok(());
                }
                KW_LET => self.parse(STMT_LET)?,
                KW_IF => self.parse(STMT_IF)?,
                _ => {
                    let checkpoint = self.checkpoint();
                    self.parse_pseudo(EXPR)?;
                    match self.consume(EQUALS | SEMI | RBRACE)? {
                        EQUALS => {
                            let mut parser = self.with_node_at(checkpoint, STMT_ASSIGN);
                            parser.parse_pseudo(EXPR)?;
                            parser.consume(SEMI)?;
                        }
                        SEMI => {
                            self.with_node_at(checkpoint, STMT_EXPR);
                        }
                        RBRACE => return Ok(()),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    fn stmt_if(&mut self) -> Result<()> {
        self.consume(KW_IF)?;
        self.parse_pseudo(EXPR)?;
        self.parse(EXPR_BLOCK)?;
        if self.consume(KW_ELSE).is_err() {
            return Ok(());
        }
        if self.expect(KW_IF | LBRACE)? == KW_IF {
            self.parse(STMT_IF)
        } else {
            self.parse(EXPR_BLOCK)
        }
    }

    fn stmt_let(&mut self) -> Result<()> {
        self.consume(KW_LET)?;
        if self.expect(KW_REC | BINDER.first())? == KW_REC {
            self.consume(KW_REC)?;
        }
        self.parse(BINDER)?;
        self.consume(EQUALS)?;
        self.parse_pseudo(EXPR)?;
        self.consume(SEMI)?;
        Ok(())
    }

    fn expr(&mut self) -> Result<()> {
        match self.peek() {
            BAR => self.parse(EXPR_CLOSURE),
            KW_IF => self.parse(EXPR_IF),
            _ => self.parse_pseudo(LEVEL_INFIX),
        }
    }

    fn expr_closure(&mut self) -> Result<()> {
        self.parse(PARAMS_CLOSURE)?;
        self.parse_pseudo(EXPR)
    }

    fn expr_if(&mut self) -> Result<()> {
        self.consume(KW_IF)?;
        self.parse_pseudo(EXPR)?;
        self.parse(EXPR_BLOCK)?;
        self.consume(KW_ELSE)?;
        if self.expect(KW_IF | LBRACE)? == KW_IF {
            self.parse(EXPR_IF)
        } else {
            self.parse(EXPR_BLOCK)
        }
    }

    fn level_infix(&mut self) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.parse_pseudo(LEVEL_PREFIX)?;
        if self.peek().is(INFIX_OPS) {
            let mut parser = self.with_node_at(checkpoint, EXPR_INFIX);
            parser.with_node(OP_INFIX).consume(INFIX_OPS)?;
            // TODO(MH): Turn tail recursion into a loop.
            parser.parse_pseudo(LEVEL_INFIX)?;
        }
        Ok(())
    }

    fn level_prefix(&mut self) -> Result<()> {
        if self.peek().is(PREFIX_OPS) {
            let mut parser = self.with_node(EXPR_PREFIX);
            parser.with_node(OP_PREFIX).consume(PREFIX_OPS)?;
            // TODO(MH): Turn tail recursion into a loop.
            parser.parse_pseudo(LEVEL_PREFIX)
        } else {
            self.parse_pseudo(LEVEL_POSTFIX)
        }
    }

    fn level_postfix(&mut self) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.parse_pseudo(LEVEL_ATOM)?;
        while self.peek().is(LPAREN | DOT) {
            match self.peek() {
                LPAREN => self.with_node_at(checkpoint, EXPR_CALL).parse(ARGS)?,
                DOT => {
                    let mut parser = self.with_node_at(checkpoint, EXPR_SELECT);
                    parser.consume(DOT)?;
                    parser.consume(LIT_NAT)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn level_atom(&mut self) -> Result<()> {
        match self.peek() {
            IDENT => {
                self.with_node(EXPR_VAR).consume(IDENT)?;
                Ok(())
            }
            LBRACE => self.parse(EXPR_BLOCK),
            LPAREN => {
                let checkpoint = self.checkpoint();
                self.consume(LPAREN)?;
                if self.expect(RPAREN | EXPR.first())? == RPAREN {
                    self.with_node_at(checkpoint, EXPR_TUPLE).consume(RPAREN)?;
                    return Ok(());
                }
                self.parse_pseudo(EXPR)?;
                if self.expect(COMMA | RPAREN)? == RPAREN {
                    self.with_node_at(checkpoint, EXPR_PAREN).consume(RPAREN)?;
                    return Ok(());
                }
                let mut parser = self.with_node_at(checkpoint, EXPR_TUPLE);
                parser.consume(COMMA)?;
                if parser.expect(EXPR.first() | RPAREN)? == RPAREN {
                    parser.consume(RPAREN)?;
                    return Ok(());
                }
                loop {
                    parser.parse_pseudo(EXPR)?;
                    if parser.consume(COMMA | RPAREN)? == RPAREN {
                        return Ok(());
                    }
                }
            }
            token if token.is(LITERALS) => {
                self.with_node(EXPR_LIT).consume(LITERALS)?;
                Ok(())
            }
            token => unreachable!("invalid first token in level_atom: {:?}", token),
        }
    }

    fn params(&mut self, ldelim: TokenKind, rdelim: TokenKind) -> Result<()> {
        self.consume(ldelim)?;
        if self.peek() == rdelim {
            self.consume(rdelim)?;
            return Ok(());
        }
        loop {
            self.parse(BINDER)?;
            if self.consume(COMMA | rdelim)? == rdelim {
                return Ok(());
            }
        }
    }

    fn binder(&mut self) -> Result<()> {
        if self.consume(KW_MUT | IDENT)? == KW_MUT {
            self.consume(IDENT)?;
        }
        Ok(())
    }

    fn args(&mut self) -> Result<()> {
        self.consume(LPAREN)?;
        if self.peek() == RPAREN {
            self.consume(RPAREN)?;
            return Ok(());
        }
        loop {
            self.parse_pseudo(EXPR)?;
            if self.consume(COMMA | RPAREN)? == RPAREN {
                return Ok(());
            }
        }
    }
}
