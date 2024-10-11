use super::*;
use felix_common::{srcloc::Mapper, Problem};

use insta::assert_snapshot;

fn parse(input: &str) -> ParseResult {
    let mapper = Mapper::new(input);
    let parser = Parser::new(input, &mapper);
    parser.run(Parser::program)
}

fn parse_expr(input: &str) -> ParseResult {
    let input = input.replace(" ", "");
    let mapper = Mapper::new(&input);
    let parser = Parser::new(&input, &mapper);
    parser.test_pseudo(first::PseudoKind::EXPR)
}

// TODO(MH): Filter out trivia.
fn dump_syntax(node: syntax::Node, _include_trivia: bool) -> String {
    format!("{:#?}", node)
}

fn dump_problems(problems: &Vec<Problem>) -> String {
    let mut buffer = String::new();
    for problem in problems {
        let Problem {
            start,
            end,
            severity,
            source,
            message,
        } = problem;
        buffer.push_str(&format!(
            "{:?} {:?}-{:?}: {} [{}]\n",
            severity, start, end, message, source
        ));
    }
    buffer
}

#[test]
fn empty() {
    let result = parse("");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..0
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn one_good_fn() {
    let result = parse("fn f() {}");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..9
      DEFN_FN@0..9
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        EXPR_BLOCK@7..9
          LBRACE@7..8 "{"
          RBRACE@8..9 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn two_good_fns() {
    let result = parse("fn f() {} fn g() {}");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..19
      DEFN_FN@0..9
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        EXPR_BLOCK@7..9
          LBRACE@7..8 "{"
          RBRACE@8..9 "}"
      WHITESPACE@9..10 " "
      DEFN_FN@10..19
        KW_FN@10..12 "fn"
        WHITESPACE@12..13 " "
        IDENT@13..14 "g"
        PARAMS_FN@14..16
          LPAREN@14..15 "("
          RPAREN@15..16 ")"
        WHITESPACE@16..17 " "
        EXPR_BLOCK@17..19
          LBRACE@17..18 "{"
          RBRACE@18..19 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn infix() {
    let result = parse("fn f(x) { x + x }");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..17
      DEFN_FN@0..17
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..7
          LPAREN@4..5 "("
          BINDER@5..6
            IDENT@5..6 "x"
          RPAREN@6..7 ")"
        WHITESPACE@7..8 " "
        EXPR_BLOCK@8..17
          LBRACE@8..9 "{"
          WHITESPACE@9..10 " "
          EXPR_INFIX@10..16
            EXPR_VAR@10..11
              IDENT@10..11 "x"
            WHITESPACE@11..12 " "
            OP_INFIX@12..13
              PLUS@12..13 "+"
            WHITESPACE@13..14 " "
            EXPR_VAR@14..15
              IDENT@14..15 "x"
            WHITESPACE@15..16 " "
          RBRACE@16..17 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn missing_infix() {
    let result = parse("fn f(x) { x x }");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..15
      DEFN_FN@0..12
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..7
          LPAREN@4..5 "("
          BINDER@5..6
            IDENT@5..6 "x"
          RPAREN@6..7 ")"
        WHITESPACE@7..8 " "
        EXPR_BLOCK@8..12
          LBRACE@8..9 "{"
          WHITESPACE@9..10 " "
          EXPR_VAR@10..11
            IDENT@10..11 "x"
          WHITESPACE@11..12 " "
      ERROR@12..15
        IDENT@12..13 "x"
        WHITESPACE@13..14 " "
        RBRACE@14..15 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @r#"
    ERROR 1:13-1:14: Found IDENT, expected RBRACE | EQUALS | SEMI. [parser/expr_block]
    "#);
}

#[test]
fn call() {
    let result = parse("fn f(x) { f(x) }");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..16
      DEFN_FN@0..16
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..7
          LPAREN@4..5 "("
          BINDER@5..6
            IDENT@5..6 "x"
          RPAREN@6..7 ")"
        WHITESPACE@7..8 " "
        EXPR_BLOCK@8..16
          LBRACE@8..9 "{"
          WHITESPACE@9..10 " "
          EXPR_CALL@10..14
            EXPR_VAR@10..11
              IDENT@10..11 "f"
            ARGS@11..14
              LPAREN@11..12 "("
              EXPR_VAR@12..13
                IDENT@12..13 "x"
              RPAREN@13..14 ")"
          WHITESPACE@14..15 " "
          RBRACE@15..16 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn one_tuple() {
    let result = parse("fn f() { (1,) }");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..15
      DEFN_FN@0..15
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        EXPR_BLOCK@7..15
          LBRACE@7..8 "{"
          WHITESPACE@8..9 " "
          EXPR_TUPLE@9..13
            LPAREN@9..10 "("
            EXPR_LIT@10..11
              LIT_NAT@10..11 "1"
            COMMA@11..12 ","
            RPAREN@12..13 ")"
          WHITESPACE@13..14 " "
          RBRACE@14..15 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn assign() {
    let result = parse("fn f() { x = 1; }");
    assert_snapshot!(dump_syntax(result.syntax, false), @r#"
    PROGRAM@0..17
      DEFN_FN@0..17
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        EXPR_BLOCK@7..17
          LBRACE@7..8 "{"
          WHITESPACE@8..9 " "
          STMT_ASSIGN@9..15
            EXPR_VAR@9..10
              IDENT@9..10 "x"
            WHITESPACE@10..11 " "
            EQUALS@11..12 "="
            WHITESPACE@12..13 " "
            EXPR_LIT@13..14
              LIT_NAT@13..14 "1"
            SEMI@14..15 ";"
          WHITESPACE@15..16 " "
          RBRACE@16..17 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

mod level_infix {
    use super::*;

    #[test]
    fn or_or() {
        let result = parse_expr("A || B || C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              BAR_BAR@1..3 "||"
            EXPR_INFIX@3..7
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..6
                BAR_BAR@4..6 "||"
              EXPR_VAR@6..7
                IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn or_and() {
        let result = parse_expr("A || B && C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              BAR_BAR@1..3 "||"
            EXPR_INFIX@3..7
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..6
                AMPER_AMPER@4..6 "&&"
              EXPR_VAR@6..7
                IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn and_or() {
        let result = parse_expr("A && B || C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_INFIX@0..4
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..3
                AMPER_AMPER@1..3 "&&"
              EXPR_VAR@3..4
                IDENT@3..4 "B"
            OP_INFIX@4..6
              BAR_BAR@4..6 "||"
            EXPR_VAR@6..7
              IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn and_and() {
        let result = parse_expr("A && B && C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              AMPER_AMPER@1..3 "&&"
            EXPR_INFIX@3..7
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..6
                AMPER_AMPER@4..6 "&&"
              EXPR_VAR@6..7
                IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn and_cmp() {
        let result = parse_expr("A && B == C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              AMPER_AMPER@1..3 "&&"
            EXPR_INFIX@3..7
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..6
                EQUALS_EQUALS@4..6 "=="
              EXPR_VAR@6..7
                IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn cmp_and() {
        let result = parse_expr("A == B && C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_INFIX@0..4
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..3
                EQUALS_EQUALS@1..3 "=="
              EXPR_VAR@3..4
                IDENT@3..4 "B"
            OP_INFIX@4..6
              AMPER_AMPER@4..6 "&&"
            EXPR_VAR@6..7
              IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn cmp_cmp() {
        let result = parse_expr("A == B != C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_INFIX@0..4
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..3
                EQUALS_EQUALS@1..3 "=="
              EXPR_VAR@3..4
                IDENT@3..4 "B"
            ERROR@4..6
              BANG_EQUALS@4..6 "!="
            EXPR_VAR@6..7
              IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:5-1:7: Cannot chain comparison operators EQUALS_EQUALS and BANG_EQUALS [parser/program]
        "#);
    }

    #[test]
    fn cmp_add() {
        let result = parse_expr("A == B + C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..6
          EXPR_INFIX@0..6
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              EQUALS_EQUALS@1..3 "=="
            EXPR_INFIX@3..6
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..5
                PLUS@4..5 "+"
              EXPR_VAR@5..6
                IDENT@5..6 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn add_cmp() {
        let result = parse_expr("A + B == C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..6
          EXPR_INFIX@0..6
            EXPR_INFIX@0..3
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..2
                PLUS@1..2 "+"
              EXPR_VAR@2..3
                IDENT@2..3 "B"
            OP_INFIX@3..5
              EQUALS_EQUALS@3..5 "=="
            EXPR_VAR@5..6
              IDENT@5..6 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn add_add() {
        let result = parse_expr("A + B - C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..5
          EXPR_INFIX@0..5
            EXPR_INFIX@0..3
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..2
                PLUS@1..2 "+"
              EXPR_VAR@2..3
                IDENT@2..3 "B"
            OP_INFIX@3..4
              MINUS@3..4 "-"
            EXPR_VAR@4..5
              IDENT@4..5 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn add_mul() {
        let result = parse_expr("A + B * C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..5
          EXPR_INFIX@0..5
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..2
              PLUS@1..2 "+"
            EXPR_INFIX@2..5
              EXPR_VAR@2..3
                IDENT@2..3 "B"
              OP_INFIX@3..4
                STAR@3..4 "*"
              EXPR_VAR@4..5
                IDENT@4..5 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn mul_add() {
        let result = parse_expr("A * B + C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..5
          EXPR_INFIX@0..5
            EXPR_INFIX@0..3
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..2
                STAR@1..2 "*"
              EXPR_VAR@2..3
                IDENT@2..3 "B"
            OP_INFIX@3..4
              PLUS@3..4 "+"
            EXPR_VAR@4..5
              IDENT@4..5 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn mul_mul() {
        let result = parse_expr("A * B / C");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..5
          EXPR_INFIX@0..5
            EXPR_INFIX@0..3
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..2
                STAR@1..2 "*"
              EXPR_VAR@2..3
                IDENT@2..3 "B"
            OP_INFIX@3..4
              SLASH@3..4 "/"
            EXPR_VAR@4..5
              IDENT@4..5 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn or_or_or() {
        let result = parse_expr("A || B || C || D");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..10
          EXPR_INFIX@0..10
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              BAR_BAR@1..3 "||"
            EXPR_INFIX@3..10
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..6
                BAR_BAR@4..6 "||"
              EXPR_INFIX@6..10
                EXPR_VAR@6..7
                  IDENT@6..7 "C"
                OP_INFIX@7..9
                  BAR_BAR@7..9 "||"
                EXPR_VAR@9..10
                  IDENT@9..10 "D"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn cmp_cmp_cmp() {
        let result = parse_expr("A < B == C > D");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..8
          EXPR_INFIX@0..8
            EXPR_INFIX@0..6
              EXPR_INFIX@0..3
                EXPR_VAR@0..1
                  IDENT@0..1 "A"
                OP_INFIX@1..2
                  LANGLE@1..2 "<"
                EXPR_VAR@2..3
                  IDENT@2..3 "B"
              ERROR@3..5
                EQUALS_EQUALS@3..5 "=="
              EXPR_VAR@5..6
                IDENT@5..6 "C"
            ERROR@6..7
              RANGLE@6..7 ">"
            EXPR_VAR@7..8
              IDENT@7..8 "D"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:4-1:6: Cannot chain comparison operators LANGLE and EQUALS_EQUALS [parser/program]
        ERROR 1:7-1:8: Cannot chain comparison operators EQUALS_EQUALS and RANGLE [parser/program]
        "#);
    }

    #[test]
    fn add_add_add() {
        let result = parse_expr("A + B + C + D");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_INFIX@0..5
              EXPR_INFIX@0..3
                EXPR_VAR@0..1
                  IDENT@0..1 "A"
                OP_INFIX@1..2
                  PLUS@1..2 "+"
                EXPR_VAR@2..3
                  IDENT@2..3 "B"
              OP_INFIX@3..4
                PLUS@3..4 "+"
              EXPR_VAR@4..5
                IDENT@4..5 "C"
            OP_INFIX@5..6
              PLUS@5..6 "+"
            EXPR_VAR@6..7
              IDENT@6..7 "D"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn or_or_err() {
        let result = parse_expr("A || B || ?");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..6
          EXPR_INFIX@0..6
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            OP_INFIX@1..3
              BAR_BAR@1..3 "||"
            EXPR_INFIX@3..6
              EXPR_VAR@3..4
                IDENT@3..4 "B"
              OP_INFIX@4..6
                BAR_BAR@4..6 "||"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:7-1:8: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LBRACE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
        "#);
    }

    #[test]
    fn add_add_err() {
        let result = parse_expr("A + B + ?");
        assert_snapshot!(dump_syntax(result.syntax, false), @r#"
        PROGRAM@0..4
          EXPR_INFIX@0..4
            EXPR_INFIX@0..3
              EXPR_VAR@0..1
                IDENT@0..1 "A"
              OP_INFIX@1..2
                PLUS@1..2 "+"
              EXPR_VAR@2..3
                IDENT@2..3 "B"
            OP_INFIX@3..4
              PLUS@3..4 "+"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:5-1:6: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LBRACE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
        "#);
    }
}
