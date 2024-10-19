use crate::parser::{Parser, Result};
use crate::syntax::{NodeKind, TokenKind, TokenKindSet};

impl<'a> Parser<'a> {
    pub(crate) fn infix(
        &mut self,
        operation_node: NodeKind,
        operand: fn(&mut Self, follow: TokenKindSet) -> Result<()>,
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
                break NodeKind::OP_INFIX;
            };
            assert!(self.peek() == op);
            self.with_node(op_node).advance();
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
            self.with_node_at(entry.checkpoint, operation_node);
        }
        res
    }
}
