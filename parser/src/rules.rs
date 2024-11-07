// This module implements a parser for the grammar provided in notes.md.
use crate::first::First;
use crate::parser::{Parser, Result};
use crate::syntax::{NodeKind, TokenKind, TokenKindSet};

use NodeKind::*;
use TokenKind::*;

impl<'a> Parser<'a> {
    pub fn program(&mut self) {
        let mut parser = self.with_root(PROGRAM);
        if let Err(problem) = parser.expr(EOF.into()) {
            parser.push_problem(problem);
        }
        parser.skip_until(EOF);
    }

    pub(crate) fn expr(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            GR_LAMBDA_LOWER => self.expr_abs(follow),
            KW_LET => self.expr_let(follow),
            token if token.starts(EXPR_APP) => self.expr_app(follow),
            token => Err(self.expecation_error(token, NodeKind::EXPR.first())),
        }
    }

    pub(crate) fn expr_abs(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(EXPR_ABS);
        parser.expect_advance(GR_LAMBDA_LOWER)?;
        parser.binder(DOT.into())?;
        parser.expect_advance(DOT)?;
        let mut parser = parser.with_node(SCOPE);
        parser.expr(follow)
    }

    pub(crate) fn expr_let(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(EXPR_LET);
        parser.expect_advance(KW_LET)?;
        parser.binder(EQUALS.into())?;
        parser.expect_advance(EQUALS)?;
        parser.expr(KW_IN.into())?;
        parser.expect_advance(KW_IN)?;
        let mut parser = parser.with_node(SCOPE);
        parser.expr(follow)
    }

    pub(crate) fn expr_app(&mut self, follow: TokenKindSet) -> Result<()> {
        let atom_first = NodeKind::EXPR_ATOM.first();
        let checkpoint = self.checkpoint();
        self.expr_atom(atom_first | follow)?;
        while self.expect(atom_first | follow)?.is(atom_first) {
            self.with_node_at(checkpoint, EXPR_APP)
                .expr_atom(atom_first | follow)?;
        }
        Ok(())
    }

    pub(crate) fn expr_atom(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            LPAREN => self.expr_paren(follow),
            ID_EXPR => self.expr_var(follow),
            KW_UNIT => self.expr_unit(follow),
            token => Err(self.expecation_error(token, NodeKind::EXPR_ATOM.first())),
        }
    }

    pub(crate) fn expr_paren(&mut self, _follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(EXPR_PAREN);
        parser.expect_advance(LPAREN)?;
        parser.expr(RPAREN.into())?;
        parser.expect_advance(RPAREN)?;
        Ok(())
    }

    pub(crate) fn expr_var(&mut self, _follow: TokenKindSet) -> Result<()> {
        self.with_node(EXPR_VAR).expect_advance(ID_EXPR)?;
        Ok(())
    }

    pub(crate) fn expr_unit(&mut self, _follow: TokenKindSet) -> Result<()> {
        self.with_node(EXPR_UNIT).expect_advance(KW_UNIT)?;
        Ok(())
    }

    pub(crate) fn r#type(&mut self, follow: TokenKindSet) -> Result<()> {
        self.type_arrow(follow)
    }

    pub(crate) fn type_arrow(&mut self, follow: TokenKindSet) -> Result<()> {
        let checkpoint = self.checkpoint();
        self.type_atom(follow)?;
        if self.expect(OP_ARROW | follow)? == OP_ARROW {
            let mut parser = self.with_node_at(checkpoint, TYPE_ARROW);
            parser.advance(OP_ARROW);
            parser.type_arrow(follow)?;
        }
        Ok(())
    }

    pub(crate) fn type_atom(&mut self, follow: TokenKindSet) -> Result<()> {
        match self.peek() {
            LPAREN => self.type_paren(follow),
            ID_TYPE => self.type_var(follow),
            TY_UNIT => self.type_unit(follow),
            token => Err(self.expecation_error(token, NodeKind::TYPE_ATOM.first())),
        }
    }

    pub(crate) fn type_paren(&mut self, _follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(TYPE_PAREN);
        parser.expect_advance(LPAREN)?;
        parser.r#type(RPAREN.into())?;
        parser.expect_advance(RPAREN)?;
        Ok(())
    }

    pub(crate) fn type_var(&mut self, _follow: TokenKindSet) -> Result<()> {
        self.with_node(TYPE_VAR).expect_advance(ID_TYPE)?;
        Ok(())
    }

    pub(crate) fn type_unit(&mut self, _follow: TokenKindSet) -> Result<()> {
        self.with_node(TYPE_UNIT).expect_advance(TY_UNIT)?;
        Ok(())
    }

    pub(crate) fn binder(&mut self, follow: TokenKindSet) -> Result<()> {
        let mut parser = self.with_node(BINDER);
        parser.with_node(NAME).expect_advance(ID_EXPR)?;
        if parser.expect(COLON | follow)? == COLON {
            parser.advance(COLON);
            parser.r#type(follow)?;
        }
        Ok(())
    }
}
