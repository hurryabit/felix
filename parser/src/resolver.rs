#![allow(dead_code)]
use felix_common::{srcloc::Mapper, Problem};
use indexmap::IndexMap;
use rowan::{ast::AstNode, GreenToken};

use crate::{
    ast,
    env::Env,
    syntax::{Node, NodeKind, NodePtr},
};

struct Resolver<'a> {
    mapper: &'a Mapper<'a>,
    env: Env<GreenToken, NodePtr>,
    symbols: IndexMap<NodePtr, ()>,
    references: IndexMap<NodePtr, NodePtr>,
    problems: Vec<Problem>,
}

#[derive(Debug, Default)]
pub struct ResolveResult {
    pub symbols: IndexMap<NodePtr, ()>,
    pub references: IndexMap<NodePtr, NodePtr>,
    pub problems: Vec<Problem>,
}

impl<'a> Resolver<'a> {
    pub fn new(mapper: &'a Mapper) -> Self {
        Self {
            mapper,
            env: Env::new(),
            symbols: IndexMap::new(),
            references: IndexMap::new(),
            problems: Vec::new(),
        }
    }

    fn enter(&mut self, node: Node) -> Option<()> {
        use NodeKind::*;

        match node.kind().into_node() {
            BINDER => {
                let node_ptr = NodePtr::new(&node);
                self.symbols.insert(node_ptr, ());
            }
            SCOPE => {
                let binder = ast::Scope::cast(node)?.binder()?;
                let token = binder.name()?.id_expr()?;
                let binder_ptr = NodePtr::new(&binder.syntax());
                self.env.push(token.green().to_owned(), binder_ptr);
            }
            EXPR_VAR => {
                let expr = ast::ExprVar::cast(node)?;
                let token = expr.id_expr()?;
                let node_ptr = NodePtr::new(&expr.syntax());
                // TODO(MH): Remove `&_.to_owned()` when (or if?)
                // https://github.com/rust-analyzer/rowan/pull/174 lands.
                if let Some(binder_ptr) = self.env.get(&token.green().to_owned()) {
                    self.references.insert(node_ptr, *binder_ptr);
                } else {
                    let range = expr.syntax().text_range();
                    let problem = Problem {
                        start: self.mapper.src_loc(range.start().into()),
                        end: self.mapper.src_loc(range.end().into()),
                        severity: felix_common::Severity::ERROR,
                        source: String::from("resolver"),
                        message: format!(
                            "cannot find `{}` in scope ({:?})",
                            token.green().text(),
                            self.env
                        ),
                    };
                    self.problems.push(problem);
                }
            }
            _ => {}
        }
        Some(())
    }

    fn leave(&mut self, _node: Node) {}

    pub fn resolve(mut self, root: &Node) -> ResolveResult {
        use rowan::WalkEvent::*;

        let mut preorder = root.preorder();
        while let Some(event) = preorder.next() {
            match event {
                Enter(node) => self.enter(node).unwrap(),
                Leave(node) => self.leave(node),
            }
        }
        ResolveResult {
            symbols: self.symbols,
            references: self.references,
            problems: self.problems,
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;
    use crate::ast::*;

    #[test]
    fn resolve_abs_bound() {
        let syntax =
            expr_abs(binder("x", None), expr_app(expr_var("x"), expr_var("x"))).into_syntax();
        let input = syntax.to_string();
        let mapper = Mapper::new(&input);
        let resolver = Resolver::new(&mapper);
        let result = resolver.resolve(&syntax);
        assert_debug_snapshot!(result, @r#"
        ResolveResult {
            symbols: {
                SyntaxNodePtr {
                    kind: BINDER,
                    range: 2..3,
                }: (),
            },
            references: {
                SyntaxNodePtr {
                    kind: EXPR_VAR,
                    range: 4..5,
                }: SyntaxNodePtr {
                    kind: BINDER,
                    range: 2..3,
                },
                SyntaxNodePtr {
                    kind: EXPR_VAR,
                    range: 5..6,
                }: SyntaxNodePtr {
                    kind: BINDER,
                    range: 2..3,
                },
            },
            problems: [],
        }
        "#);
    }
}
