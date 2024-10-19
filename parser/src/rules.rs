// This module implements a parser for the grammar provided in notes.md.
use crate::first::First;
use crate::parser::{Parser, Result};
use crate::syntax::{
    AliasKind, NodeKind, TokenKind, TokenKindSet, BUILTIN_TYPES, EXPR_INFIX_OPS, EXPR_PREFIX_OPS,
    LITERALS, TYPE_INFIX_OPS, TYPE_PREFIX_OPS,
};

use AliasKind::*;
use NodeKind::*;
use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn program(&mut self) {
        let first = DEFN.first() | EOF;
        let mut parser = self.with_root(PROGRAM);
        while parser.peek() != EOF {
            if let Err(problem) = parser.defn(first) {
                parser.push_problem(problem);
                parser.skip_until(first);
            }
        }
    }

    pub(crate) fn defn(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            KW_TYPE => self.defn_type(follow),
            KW_LET => self.defn_let(follow),
            token => Err(self.expecation_error(token, DEFN.first())),
        }
    }

    pub(crate) fn defn_type(&mut self, follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(KW_TYPE)?;
        match self.peek() {
            KW_REC => {
                let mut parser = self.with_node_at(checkpoint, DEFN_TYPE_REC);
                parser.advance(KW_REC);
                parser.bind_type(KW_AND | follow)?;
                while parser.expect(KW_AND | follow)? == KW_AND {
                    parser.advance(KW_AND);
                    parser.bind_type(KW_AND | follow)?;
                }
                Ok(())
            }
            token if token.starts(BIND_TYPE) => {
                self.with_node_at(checkpoint, DEFN_TYPE).bind_type(follow)
            }
            token => Err(self.expecation_error(token, KW_REC | BIND_TYPE.first())),
        }
    }

    pub(crate) fn bind_type(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(BIND_TYPE);
        parser.expect_advance(IDENT)?;
        parser.expect_advance(EQ)?;
        parser.type_(follow)
    }

    pub(crate) fn type_(&mut self, follow: TokenKindSet) -> Result<()> {
        self.level_type_infix(follow)
    }

    pub(crate) fn level_type_infix(&mut self, follow: TokenKindSet) -> Result<()> {
        fn power(op: TokenKind) -> (u32, u32) {
            match op {
                ARROW => (15, 10),
                UNION => (25, 20),
                INTER => (35, 30),
                TIMES => (45, 40),
                token => unreachable!("{} is not in TYPE_INFIX_OPS", token),
            }
        }

        self.infix(
            TYPE_INFIX,
            Self::level_type_prefix,
            OP_TYPE_INFIX,
            TYPE_INFIX_OPS,
            power,
            follow,
        )
    }

    pub(crate) fn level_type_prefix(&mut self, follow: TokenKindSet) -> Result<()> {
        self.prefix(
            TYPE_PREFIX,
            Self::level_type_atom,
            LEVEL_TYPE_ATOM.first(),
            OP_TYPE_PREFIX,
            TYPE_PREFIX_OPS,
            follow,
        )
    }

    pub(crate) fn level_type_atom(&mut self, _follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            IDENT => {
                self.with_node(TYPE_REF).advance(IDENT);
                Ok(())
            }
            LPAREN => {
                let mut parser = self.with_node(TYPE_PAREN);
                parser.advance(LPAREN);
                parser.type_(RPAREN.into())?;
                parser.expect_advance(RPAREN)?;
                Ok(())
            }
            token if token.is(BUILTIN_TYPES) => {
                self.with_node(TYPE_BUILTIN).advance(BUILTIN_TYPES);
                Ok(())
            }
            token => Err(self.expecation_error(token, LEVEL_TYPE_ATOM.first())),
        }
    }

    pub(crate) fn defn_let(&mut self, follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(KW_LET)?;
        match self.peek() {
            KW_REC => {
                let mut parser = self.with_node_at(checkpoint, DEFN_LET_REC);
                parser.advance(KW_REC);
                parser.bind_expr(KW_AND | follow)?;
                while parser.expect(KW_AND | follow)? == KW_AND {
                    parser.advance(KW_AND);
                    parser.bind_expr(KW_AND | follow)?;
                }
                Ok(())
            }
            token if token.starts(BIND_EXPR) => {
                self.with_node_at(checkpoint, DEFN_LET).bind_expr(follow)
            }
            token => Err(self.expecation_error(token, KW_REC | BIND_EXPR.first())),
        }
    }

    pub(crate) fn bind_expr(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(BIND_EXPR);
        parser.pat(COLON | EQ)?;
        if parser.expect(COLON | EQ)? == COLON {
            parser.advance(COLON);
            parser.type_(EQ.into())?;
        }
        parser.expect_advance(EQ)?;
        parser.expr(follow)
    }

    pub(crate) fn pat(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            IDENT => {
                self.with_node(PAT_IDENT).advance(IDENT);
                Ok(())
            }
            LPAREN => self.pat_lparen(follow),
            token => Err(self.expecation_error(token, PAT.first())),
        }
    }

    pub(crate) fn pat_lparen(&mut self, _follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(LPAREN)?;
        if self.expect(PAT.first() | RPAREN)? == RPAREN {
            self.with_node_at(checkpoint, PAT_UNIT).advance(RPAREN);
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, PAT_PAIR);
        parser.pat(COMMA.into())?;
        parser.expect_advance(COMMA)?;
        parser.pat(RPAREN.into())?;
        parser.expect_advance(RPAREN)?;
        Ok(())
    }

    pub(crate) fn expr(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            KW_FUN => self.expr_fun(follow),
            KW_LET => self.expr_let(follow),
            KW_IF => self.expr_if(follow),
            token if token.starts(LEVEL_EXPR_INFIX) => self.level_expr_infix(follow),
            token => Err(self.expecation_error(token, EXPR.first())),
        }
    }

    pub(crate) fn expr_fun(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(EXPR_FUN);
        parser.expect_advance(KW_FUN)?;
        parser.pat(ARROW.into())?;
        parser.expect_advance(ARROW)?;
        parser.expr(follow)
    }

    pub(crate) fn expr_let(&mut self, follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(KW_LET)?;
        match self.peek() {
            KW_REC => {
                let mut parser = self.with_node_at(checkpoint, EXPR_LET_REC);
                parser.advance(KW_REC);
                parser.bind_expr(KW_AND | KW_IN)?;
                while parser.expect(KW_AND | KW_IN)? == KW_AND {
                    parser.advance(KW_AND);
                    parser.bind_expr(KW_AND | KW_IN)?;
                }
                parser.expect_advance(KW_IN)?;
                parser.expr(follow)
            }
            token if token.starts(BIND_EXPR) => {
                let mut parser = self.with_node_at(checkpoint, EXPR_LET);
                parser.bind_expr(KW_IN.into())?;
                parser.expect_advance(KW_IN)?;
                parser.expr(follow)
            }
            token => Err(self.expecation_error(token, KW_REC | BIND_EXPR.first())),
        }
    }

    pub(crate) fn expr_if(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(EXPR_IF);
        parser.expect_advance(KW_IF)?;
        parser.expr(KW_THEN.into())?;
        parser.expect_advance(KW_THEN)?;
        parser.expr(KW_ELSE.into())?;
        parser.expect_advance(KW_ELSE)?;
        parser.expr(follow)
    }

    pub(crate) fn level_expr_infix(&mut self, follow: TokenKindSet) -> Result<()> {
        fn power(op: TokenKind) -> (u32, u32) {
            match op {
                OR => (15, 10),
                AND => (25, 20),
                EQ_EQ | NOT_EQ | LT | LT_EQ | GT | GT_EQ => (30, 30),
                PLUS | MINUS => (40, 45),
                TIMES | DIV | MOD => (50, 55),
                token => unreachable!("{} is not in EXPR_INFIX_OPS", token),
            }
        }

        self.infix(
            EXPR_INFIX,
            Self::level_expr_prefix,
            OP_EXPR_INFIX,
            EXPR_INFIX_OPS,
            power,
            follow,
        )
    }

    pub(crate) fn level_expr_prefix(&mut self, follow: TokenKindSet) -> Result<()> {
        self.prefix(
            EXPR_PREFIX,
            Self::level_expr_app,
            LEVEL_EXPR_APP.first(),
            OP_EXPR_PREFIX,
            EXPR_PREFIX_OPS,
            follow,
        )
    }

    pub(crate) fn level_expr_app(&mut self, follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.level_expr_atom(LEVEL_EXPR_ATOM.first() | follow)?;
        while self.peek().starts(LEVEL_EXPR_ATOM) {
            self.with_node_at(checkpoint, EXPR_APP)
                .level_expr_atom(LEVEL_EXPR_ATOM.first() | follow)?;
        }
        Ok(())
    }

    pub(crate) fn level_expr_atom(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            token if token.is(LITERALS) => {
                self.with_node(EXPR_LIT).advance(LITERALS);
                Ok(())
            }
            IDENT => {
                self.with_node(EXPR_REF).advance(IDENT);
                Ok(())
            }
            LPAREN => self.expr_lparen(follow),
            token => Err(self.expecation_error(token, LEVEL_EXPR_ATOM.first())),
        }
    }

    pub(crate) fn expr_lparen(&mut self, _follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.expect_advance(LPAREN)?;
        if self.expect(EXPR.first() | RPAREN)? == RPAREN {
            self.with_node_at(checkpoint, EXPR_UNIT).advance(RPAREN);
            return Ok(());
        }
        self.expr(COMMA | RPAREN)?;
        if self.expect(COMMA | RPAREN)? == RPAREN {
            self.with_node_at(checkpoint, EXPR_PAREN).advance(RPAREN);
            return Ok(());
        }
        let mut parser = self.with_node_at(checkpoint, EXPR_PAIR);
        parser.advance(COMMA);
        parser.expr(RPAREN.into())?;
        parser.expect_advance(RPAREN)?;
        Ok(())
    }
}
