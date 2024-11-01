use crate::parser::{Parser, Result};
use crate::syntax::{NodeKind, TokenKind, TokenKindSet};

impl<'a> Parser<'a> {
    #[allow(dead_code)]
    pub(crate) fn infix(
        &mut self,
        operation_node: NodeKind,
        operand: fn(&mut Self, follow: TokenKindSet) -> Result<()>,
        operator_node: NodeKind,
        operators: TokenKindSet,
        operator_power: fn(TokenKind) -> (u32, u32),
        follow: TokenKindSet,
    ) -> Result<()> {
        // NOTE(MH): We use Pratt parsing to resolve precendence. We use
        // matklad's trick of different binding powers on the left and right
        // to resolve associativity. See
        // https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
        #[derive(Clone, Copy)]
        struct StackEntry {
            checkpoint: rowan::Checkpoint,
            op: TokenKind,
            right_power: u32,
        }

        let mut stack: Vec<StackEntry> = Vec::new();
        let mut checkpoint = self.checkpoint();
        operand(self, operators | follow)?;

        let res = loop {
            let op = self.expect(operators | follow)?;
            if !op.is(operators) {
                break Ok(());
            }
            let (left_power, right_power) = operator_power(op);
            let op_node = loop {
                if let Some(top) = stack.last().copied() {
                    if top.right_power >= left_power {
                        checkpoint = top.checkpoint;
                        self.with_node_at(top.checkpoint, operation_node);
                        stack.pop();
                        if top.right_power > left_power {
                            continue;
                        }
                        let problem =
                            self.error(format!("Cannot chain operators {} and {}", top.op, op));
                        self.push_problem(problem);
                        break NodeKind::ERROR;
                    }
                }
                break operator_node;
            };
            self.with_node(op_node).advance(op);
            stack.push(StackEntry {
                checkpoint,
                op,
                right_power,
            });
            checkpoint = self.checkpoint();
            if let Err(problem) = operand(self, follow) {
                break Err(problem);
            }
        };
        for entry in stack.into_iter().rev() {
            self.with_node_at(entry.checkpoint, operation_node);
        }
        res
    }

    #[allow(dead_code)]
    pub(crate) fn prefix(
        &mut self,
        operation_node: NodeKind,
        operand: fn(&mut Self, follow: TokenKindSet) -> Result<()>,
        operand_first: TokenKindSet,
        operator_node: NodeKind,
        operators: TokenKindSet,
        follow: TokenKindSet,
    ) -> Result<()> {
        let mut stack: Vec<rowan::Checkpoint> = Vec::new();
        let res = loop {
            match self.peek() {
                token if token.is(operators) => {
                    stack.push(self.checkpoint());
                    self.with_node(operator_node).advance(token);
                }
                token if token.is(operand_first) => break operand(self, follow),
                token => break Err(self.expecation_error(token, operators | operand_first)),
            }
        };
        for checkpoint in stack.into_iter().rev() {
            self.with_node_at(checkpoint, operation_node);
        }
        res
    }
}
