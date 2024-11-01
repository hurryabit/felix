use assert_matches::assert_matches;

use felix_common::{srcloc::Mapper, SrcLoc};

use super::{ast::*, *};

struct SuccessCase {
    name: &'static str,
    input: &'static str,
    expect: ast::GreenChild,
}

struct FailureCase {
    name: &'static str,
    input: &'static str,
    start: SrcLoc,
    source: &'static str,
}

#[test]
fn expr_success() {
    let cases = vec![
        // Basic cases
        SuccessCase {
            name: "abs_annot",
            input: "λx:T.e",
            expect: expr_abs(binder("x", Some(type_var("T"))), expr_var("e")),
        },
        SuccessCase {
            name: "abs_plain",
            input: "λx.e",
            expect: expr_abs(binder("x", None), expr_var("e")),
        },
        SuccessCase {
            name: "app",
            input: "e1 e2",
            expect: expr_app(expr_var("e1"), expr_var("e2")),
        },
        SuccessCase {
            name: "let_annot",
            input: "let x:T = e1 in e2",
            expect: expr_let(
                binder("x", Some(type_var("T"))),
                expr_var("e1"),
                expr_var("e2"),
            ),
        },
        SuccessCase {
            name: "let_plain",
            input: "let x = e1 in e2",
            expect: expr_let(binder("x", None), expr_var("e1"), expr_var("e2")),
        },
        SuccessCase {
            name: "paren",
            input: "(e)",
            expect: expr_paren(expr_var("e")),
        },
        SuccessCase {
            name: "var",
            input: "x",
            expect: expr_var("x"),
        },
        SuccessCase {
            name: "unit",
            input: "unit",
            expect: expr_unit(),
        },
        // Advanced cases
        SuccessCase {
            name: "abs_abs",
            input: "λx1.λx2.e",
            expect: expr_abs(
                binder("x1", None),
                expr_abs(binder("x2", None), expr_var("e")),
            ),
        },
        SuccessCase {
            name: "app_app",
            input: "e1 e2 e3",
            expect: expr_app(expr_app(expr_var("e1"), expr_var("e2")), expr_var("e3")),
        },
        SuccessCase {
            name: "abs_app",
            input: "λx.e1 e2",
            expect: expr_abs(binder("x", None), expr_app(expr_var("e1"), expr_var("e2"))),
        },
        SuccessCase {
            name: "app_abs_fun",
            input: "(λx.e1) e2",
            expect: expr_app(
                expr_paren(expr_abs(binder("x", None), expr_var("e1"))),
                expr_var("e2"),
            ),
        },
        SuccessCase {
            name: "app_abs_arg",
            input: "e1 (λx.e2)",
            expect: expr_app(
                expr_var("e1"),
                expr_paren(expr_abs(binder("x", None), expr_var("e2"))),
            ),
        },
        // WebUI examples
        SuccessCase {
            name: "twice",
            input: r#"
                # Simple higher order function
                let twice =
                    λf:Unit -> Unit. λu:Unit. f (f u)
                in
                twice (λu:Unit. u) unit
            "#,
            expect: expr_let(
                binder("twice", None),
                expr_abs(
                    binder("f", Some(type_arrow(type_unit(), type_unit()))),
                    expr_abs(
                        binder("u", Some(type_unit())),
                        expr_app(
                            expr_var("f"),
                            expr_paren(expr_app(expr_var("f"), expr_var("u"))),
                        ),
                    ),
                ),
                expr_app(
                    expr_app(
                        expr_var("twice"),
                        expr_paren(expr_abs(binder("u", Some(type_unit())), expr_var("u"))),
                    ),
                    expr_unit(),
                ),
            ),
        },
    ];
    for case in cases {
        let mapper = Mapper::new(&case.input);
        let parser = Parser::new(&case.input, &mapper).without_trivia();
        let result = parser.run_partial(Parser::expr);
        assert_matches!(&result.problems[..], [], "test case {}", case.name);
        let parsed = format!("{:#?}", result.syntax);
        let expected = format!("{:#?}", case.expect.into_syntax());
        pretty_assertions::assert_eq!(parsed, expected, "test case {}", case.name);
    }
}

#[test]
fn expr_failure() {
    let cases = vec![
        FailureCase {
            name: "abs_as_arg",
            input: "e1 λx.e2",
            start: SrcLoc::new(0, 3),
            source: "parser/error", // TODO(MH): Better error sources.
        },
        FailureCase {
            name: "let_as_arg",
            input: "e1 let x = e2 in e3",
            start: SrcLoc::new(0, 3),
            source: "parser/error",
        },
    ];
    for case in cases {
        let mapper = Mapper::new(&case.input);
        let parser = Parser::new(&case.input, &mapper).without_trivia();
        let result = parser.run_partial(Parser::expr);
        assert_matches!(&result.problems[..], [_], "test case {}", case.name);
        let problem = &result.problems[0];
        assert_eq!(
            problem.start, case.start,
            "test case {} ({})",
            case.name, problem.message
        );
        assert_eq!(
            problem.source, case.source,
            "test case {} ({})",
            case.name, problem.message
        );
    }
}

#[test]
fn type_success() {
    let cases = vec![
        // Basic cases
        SuccessCase {
            name: "arrow",
            input: "T1 -> T2",
            expect: type_arrow(type_var("T1"), type_var("T2")),
        },
        SuccessCase {
            name: "var",
            input: "A",
            expect: type_var("A"),
        },
        SuccessCase {
            name: "unit",
            input: "Unit",
            expect: type_unit(),
        },
        // Advanced cases
        SuccessCase {
            name: "arrow_associativity",
            input: "T1 -> T2 -> T3",
            expect: type_arrow(type_var("T1"), type_arrow(type_var("T2"), type_var("T3"))),
        },
    ];
    for case in cases {
        let mapper = Mapper::new(&case.input);
        let parser = Parser::new(&case.input, &mapper).without_trivia();
        let result = parser.run_partial(Parser::r#type);
        assert_matches!(&result.problems[..], [], "test case {}", case.name);
        let parsed = format!("{:#?}", result.syntax);
        let expected = format!("{:#?}", case.expect.into_syntax());
        pretty_assertions::assert_eq!(parsed, expected, "test case {}", case.name);
    }
}

#[test]
fn type_failure() {
    let cases = vec![FailureCase {
        name: "no_app",
        input: "T1 T2",
        start: SrcLoc::new(0, 3),
        source: "parser/error", // TODO(MH): Better error sources.
    }];
    for case in cases {
        let mapper = Mapper::new(&case.input);
        let parser = Parser::new(&case.input, &mapper).without_trivia();
        let result = parser.run_partial(Parser::r#type);
        assert_matches!(&result.problems[..], [_], "test case {}", case.name);
        let problem = &result.problems[0];
        assert_eq!(
            problem.start, case.start,
            "test case {} ({})",
            case.name, problem.message
        );
        assert_eq!(
            problem.source, case.source,
            "test case {} ({})",
            case.name, problem.message
        );
    }
}
