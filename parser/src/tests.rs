use super::*;
use felix_common::{srcloc::Mapper, Problem};

use insta::{assert_debug_snapshot, assert_snapshot};

fn parse(input: &str) -> ParseResult {
    let mapper = Mapper::new(input);
    let parser = Parser::new(input, &mapper);
    parser.run(Parser::program)
}

fn parse_success(input: &str) -> syntax::Node {
    let result = parse(input);
    assert!(result.problems.is_empty());
    result.syntax
}

fn parse_expr(input: &str) -> ParseResult {
    fn expr(parser: &mut Parser) {
        let mut parser = parser.with_root(syntax::NodeKind::PROGRAM);
        if let Err(problem) = parser.expr(syntax::TokenKind::EOF) {
            parser.push_problem(problem);
        }
        parser.skip_until(syntax::TokenKind::EOF);
    }

    let input = input.replace(" ", "");
    let mapper = Mapper::new(&input);
    let parser = Parser::new(&input, &mapper);
    parser.run(expr)
}

fn parse_expr_success(input: &str) -> syntax::Node {
    let result = parse_expr(input);
    assert!(result.problems.is_empty());
    result.syntax
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
fn webui_sample() {
    let result = parse(
        r#"
    // Recursive version of Fibonacci.
    fn fib_rec(n) {
        if n <= 1 {
            n
        } else {
            fib_rec(n-1) + fib_rec(n-1)
        }
    }

    // Loop-via-tail-recusrsion version of Fibonacci.
    fn fib_tailrec(n) {
        let mut a = 0;
        let mut b = 1;
        let rec go = |n| {
            if n > 0 {
                let c = b;
                b = a + b;
                a = c;
                go(n-1);
            }
        };
        go(n);
        a
    }
    "#,
    );
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..504
      WHITESPACE@0..5 "\n    "
      COMMENT@5..40 "// Recursive version  ..."
      WHITESPACE@40..44 "    "
      DEFN_FN@44..166
        KW_FN@44..46 "fn"
        WHITESPACE@46..47 " "
        IDENT@47..54 "fib_rec"
        PARAMS_FN@54..57
          LPAREN@54..55 "("
          BINDER@55..56
            IDENT@55..56 "n"
          RPAREN@56..57 ")"
        WHITESPACE@57..58 " "
        BLOCK@58..166
          LBRACE@58..59 "{"
          WHITESPACE@59..68 "\n        "
          STMT_IF@68..160
            KW_IF@68..70 "if"
            WHITESPACE@70..71 " "
            EXPR_INFIX@71..77
              EXPR_VAR@71..72
                IDENT@71..72 "n"
              WHITESPACE@72..73 " "
              OP_INFIX@73..75
                LANGLE_EQUALS@73..75 "<="
              WHITESPACE@75..76 " "
              EXPR_LIT@76..77
                LIT_NAT@76..77 "1"
            WHITESPACE@77..78 " "
            BLOCK@78..103
              LBRACE@78..79 "{"
              WHITESPACE@79..92 "\n            "
              EXPR_VAR@92..93
                IDENT@92..93 "n"
              WHITESPACE@93..102 "\n        "
              RBRACE@102..103 "}"
            WHITESPACE@103..104 " "
            KW_ELSE@104..108 "else"
            WHITESPACE@108..109 " "
            BLOCK@109..160
              LBRACE@109..110 "{"
              WHITESPACE@110..123 "\n            "
              EXPR_INFIX@123..150
                EXPR_CALL@123..135
                  EXPR_VAR@123..130
                    IDENT@123..130 "fib_rec"
                  ARGS@130..135
                    LPAREN@130..131 "("
                    EXPR_INFIX@131..134
                      EXPR_VAR@131..132
                        IDENT@131..132 "n"
                      OP_INFIX@132..133
                        MINUS@132..133 "-"
                      EXPR_LIT@133..134
                        LIT_NAT@133..134 "1"
                    RPAREN@134..135 ")"
                WHITESPACE@135..136 " "
                OP_INFIX@136..137
                  PLUS@136..137 "+"
                WHITESPACE@137..138 " "
                EXPR_CALL@138..150
                  EXPR_VAR@138..145
                    IDENT@138..145 "fib_rec"
                  ARGS@145..150
                    LPAREN@145..146 "("
                    EXPR_INFIX@146..149
                      EXPR_VAR@146..147
                        IDENT@146..147 "n"
                      OP_INFIX@147..148
                        MINUS@147..148 "-"
                      EXPR_LIT@148..149
                        LIT_NAT@148..149 "1"
                    RPAREN@149..150 ")"
              WHITESPACE@150..159 "\n        "
              RBRACE@159..160 "}"
          WHITESPACE@160..165 "\n    "
          RBRACE@165..166 "}"
      WHITESPACE@166..172 "\n\n    "
      COMMENT@172..222 "// Loop-via-tail-recu ..."
      WHITESPACE@222..226 "    "
      DEFN_FN@226..499
        KW_FN@226..228 "fn"
        WHITESPACE@228..229 " "
        IDENT@229..240 "fib_tailrec"
        PARAMS_FN@240..243
          LPAREN@240..241 "("
          BINDER@241..242
            IDENT@241..242 "n"
          RPAREN@242..243 ")"
        WHITESPACE@243..244 " "
        BLOCK@244..499
          LBRACE@244..245 "{"
          WHITESPACE@245..254 "\n        "
          STMT_LET@254..268
            KW_LET@254..257 "let"
            WHITESPACE@257..258 " "
            BINDER@258..263
              KW_MUT@258..261 "mut"
              WHITESPACE@261..262 " "
              IDENT@262..263 "a"
            WHITESPACE@263..264 " "
            EQUALS@264..265 "="
            WHITESPACE@265..266 " "
            EXPR_LIT@266..267
              LIT_NAT@266..267 "0"
            SEMI@267..268 ";"
          WHITESPACE@268..277 "\n        "
          STMT_LET@277..291
            KW_LET@277..280 "let"
            WHITESPACE@280..281 " "
            BINDER@281..286
              KW_MUT@281..284 "mut"
              WHITESPACE@284..285 " "
              IDENT@285..286 "b"
            WHITESPACE@286..287 " "
            EQUALS@287..288 "="
            WHITESPACE@288..289 " "
            EXPR_LIT@289..290
              LIT_NAT@289..290 "1"
            SEMI@290..291 ";"
          WHITESPACE@291..300 "\n        "
          STMT_LET@300..468
            KW_LET@300..303 "let"
            WHITESPACE@303..304 " "
            KW_REC@304..307 "rec"
            WHITESPACE@307..308 " "
            BINDER@308..310
              IDENT@308..310 "go"
            WHITESPACE@310..311 " "
            EQUALS@311..312 "="
            WHITESPACE@312..313 " "
            EXPR_CLOSURE@313..467
              PARAMS_CLOSURE@313..316
                BAR@313..314 "|"
                BINDER@314..315
                  IDENT@314..315 "n"
                BAR@315..316 "|"
              WHITESPACE@316..317 " "
              BLOCK@317..467
                LBRACE@317..318 "{"
                WHITESPACE@318..331 "\n            "
                STMT_IF@331..457
                  KW_IF@331..333 "if"
                  WHITESPACE@333..334 " "
                  EXPR_INFIX@334..339
                    EXPR_VAR@334..335
                      IDENT@334..335 "n"
                    WHITESPACE@335..336 " "
                    OP_INFIX@336..337
                      RANGLE@336..337 ">"
                    WHITESPACE@337..338 " "
                    EXPR_LIT@338..339
                      LIT_NAT@338..339 "0"
                  WHITESPACE@339..340 " "
                  BLOCK@340..457
                    LBRACE@340..341 "{"
                    WHITESPACE@341..358 "\n                "
                    STMT_LET@358..368
                      KW_LET@358..361 "let"
                      WHITESPACE@361..362 " "
                      BINDER@362..363
                        IDENT@362..363 "c"
                      WHITESPACE@363..364 " "
                      EQUALS@364..365 "="
                      WHITESPACE@365..366 " "
                      EXPR_VAR@366..367
                        IDENT@366..367 "b"
                      SEMI@367..368 ";"
                    WHITESPACE@368..385 "\n                "
                    STMT_ASSIGN@385..395
                      EXPR_VAR@385..386
                        IDENT@385..386 "b"
                      WHITESPACE@386..387 " "
                      EQUALS@387..388 "="
                      WHITESPACE@388..389 " "
                      EXPR_INFIX@389..394
                        EXPR_VAR@389..390
                          IDENT@389..390 "a"
                        WHITESPACE@390..391 " "
                        OP_INFIX@391..392
                          PLUS@391..392 "+"
                        WHITESPACE@392..393 " "
                        EXPR_VAR@393..394
                          IDENT@393..394 "b"
                      SEMI@394..395 ";"
                    WHITESPACE@395..412 "\n                "
                    STMT_ASSIGN@412..418
                      EXPR_VAR@412..413
                        IDENT@412..413 "a"
                      WHITESPACE@413..414 " "
                      EQUALS@414..415 "="
                      WHITESPACE@415..416 " "
                      EXPR_VAR@416..417
                        IDENT@416..417 "c"
                      SEMI@417..418 ";"
                    WHITESPACE@418..435 "\n                "
                    STMT_EXPR@435..443
                      EXPR_CALL@435..442
                        EXPR_VAR@435..437
                          IDENT@435..437 "go"
                        ARGS@437..442
                          LPAREN@437..438 "("
                          EXPR_INFIX@438..441
                            EXPR_VAR@438..439
                              IDENT@438..439 "n"
                            OP_INFIX@439..440
                              MINUS@439..440 "-"
                            EXPR_LIT@440..441
                              LIT_NAT@440..441 "1"
                          RPAREN@441..442 ")"
                      SEMI@442..443 ";"
                    WHITESPACE@443..456 "\n            "
                    RBRACE@456..457 "}"
                WHITESPACE@457..466 "\n        "
                RBRACE@466..467 "}"
            SEMI@467..468 ";"
          WHITESPACE@468..477 "\n        "
          STMT_EXPR@477..483
            EXPR_CALL@477..482
              EXPR_VAR@477..479
                IDENT@477..479 "go"
              ARGS@479..482
                LPAREN@479..480 "("
                EXPR_VAR@480..481
                  IDENT@480..481 "n"
                RPAREN@481..482 ")"
            SEMI@482..483 ";"
          WHITESPACE@483..492 "\n        "
          EXPR_VAR@492..493
            IDENT@492..493 "a"
          WHITESPACE@493..498 "\n    "
          RBRACE@498..499 "}"
      WHITESPACE@499..504 "\n    "
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn empty() {
    let result = parse("");
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..0
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn one_good_fn() {
    let result = parse("fn f() {}");
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..9
      DEFN_FN@0..9
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        BLOCK@7..9
          LBRACE@7..8 "{"
          RBRACE@8..9 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn two_good_fns() {
    let result = parse("fn f() {} fn g() {}");
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..19
      DEFN_FN@0..9
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        BLOCK@7..9
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
        BLOCK@17..19
          LBRACE@17..18 "{"
          RBRACE@18..19 "}"
    "#);
    assert_snapshot!(dump_problems(&result.problems), @"");
}

#[test]
fn one_good_fn_between_errors() {
    let result = parse(" $ fn f() {} $ ");
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..15
      WHITESPACE@0..1 " "
      ERROR@1..2
        UNKNOWN@1..2 "$"
      WHITESPACE@2..3 " "
      DEFN_FN@3..12
        KW_FN@3..5 "fn"
        WHITESPACE@5..6 " "
        IDENT@6..7 "f"
        PARAMS_FN@7..9
          LPAREN@7..8 "("
          RPAREN@8..9 ")"
        WHITESPACE@9..10 " "
        BLOCK@10..12
          LBRACE@10..11 "{"
          RBRACE@11..12 "}"
      WHITESPACE@12..13 " "
      ERROR@13..14
        UNKNOWN@13..14 "$"
      WHITESPACE@14..15 " "
    "#);
    assert_snapshot!(dump_problems(&result.problems), @r#"
    ERROR 1:2-1:3: Found UNKNOWN, expected KW_FN. [parser/program]
    ERROR 1:14-1:15: Found UNKNOWN, expected KW_FN. [parser/program]
    "#);
}

#[test]
fn infix() {
    let result = parse("fn f(x) { x + x }");
    assert_debug_snapshot!(result.syntax, @r#"
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
        BLOCK@8..17
          LBRACE@8..9 "{"
          WHITESPACE@9..10 " "
          EXPR_INFIX@10..15
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
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..15
      DEFN_FN@0..11
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..7
          LPAREN@4..5 "("
          BINDER@5..6
            IDENT@5..6 "x"
          RPAREN@6..7 ")"
        WHITESPACE@7..8 " "
        BLOCK@8..11
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
    ERROR 1:13-1:14: Found IDENT, expected RBRACE | EQUALS | QUERY | SEMI. [parser/block]
    "#);
}

#[test]
fn call() {
    let result = parse("fn f(x) { f(x) }");
    assert_debug_snapshot!(result.syntax, @r#"
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
        BLOCK@8..16
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
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..15
      DEFN_FN@0..15
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        BLOCK@7..15
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
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..17
      DEFN_FN@0..17
        KW_FN@0..2 "fn"
        WHITESPACE@2..3 " "
        IDENT@3..4 "f"
        PARAMS_FN@4..6
          LPAREN@4..5 "("
          RPAREN@5..6 ")"
        WHITESPACE@6..7 " "
        BLOCK@7..17
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

mod defn_fn {
    use super::*;

    #[test]
    fn no_params() {
        let syntax = parse_success("fn f() {}");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..9
          DEFN_FN@0..9
            KW_FN@0..2 "fn"
            WHITESPACE@2..3 " "
            IDENT@3..4 "f"
            PARAMS_FN@4..6
              LPAREN@4..5 "("
              RPAREN@5..6 ")"
            WHITESPACE@6..7 " "
            BLOCK@7..9
              LBRACE@7..8 "{"
              RBRACE@8..9 "}"
        "#);
    }

    #[test]
    fn one_param() {
        let syntax = parse_success("fn f(x) {}");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..10
          DEFN_FN@0..10
            KW_FN@0..2 "fn"
            WHITESPACE@2..3 " "
            IDENT@3..4 "f"
            PARAMS_FN@4..7
              LPAREN@4..5 "("
              BINDER@5..6
                IDENT@5..6 "x"
              RPAREN@6..7 ")"
            WHITESPACE@7..8 " "
            BLOCK@8..10
              LBRACE@8..9 "{"
              RBRACE@9..10 "}"
        "#);
    }

    #[test]
    fn two_params() {
        let syntax = parse_success("fn f(x, y) {}");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..13
          DEFN_FN@0..13
            KW_FN@0..2 "fn"
            WHITESPACE@2..3 " "
            IDENT@3..4 "f"
            PARAMS_FN@4..10
              LPAREN@4..5 "("
              BINDER@5..6
                IDENT@5..6 "x"
              COMMA@6..7 ","
              WHITESPACE@7..8 " "
              BINDER@8..9
                IDENT@8..9 "y"
              RPAREN@9..10 ")"
            WHITESPACE@10..11 " "
            BLOCK@11..13
              LBRACE@11..12 "{"
              RBRACE@12..13 "}"
        "#);
    }
}

mod level_tertiary {
    use super::*;

    #[test]
    fn simple() {
        let syntax = parse_expr_success("A ? B : C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          EXPR_TERTIARY@0..5
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            QUERY@1..2 "?"
            EXPR_VAR@2..3
              IDENT@2..3 "B"
            COLON@3..4 ":"
            EXPR_VAR@4..5
              IDENT@4..5 "C"
        "#);
    }

    #[test]
    fn precedence() {
        let syntax = parse_expr_success("A1 || A2 ? B1 || B2 : C1 || C2");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..20
          EXPR_TERTIARY@0..20
            EXPR_INFIX@0..6
              EXPR_VAR@0..2
                IDENT@0..2 "A1"
              OP_INFIX@2..4
                BAR_BAR@2..4 "||"
              EXPR_VAR@4..6
                IDENT@4..6 "A2"
            QUERY@6..7 "?"
            EXPR_INFIX@7..13
              EXPR_VAR@7..9
                IDENT@7..9 "B1"
              OP_INFIX@9..11
                BAR_BAR@9..11 "||"
              EXPR_VAR@11..13
                IDENT@11..13 "B2"
            COLON@13..14 ":"
            EXPR_INFIX@14..20
              EXPR_VAR@14..16
                IDENT@14..16 "C1"
              OP_INFIX@16..18
                BAR_BAR@16..18 "||"
              EXPR_VAR@18..20
                IDENT@18..20 "C2"
        "#);
    }

    #[test]
    fn right_associative() {
        let syntax = parse_expr_success("A ? B : C ? D : E");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..9
          EXPR_TERTIARY@0..9
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            QUERY@1..2 "?"
            EXPR_VAR@2..3
              IDENT@2..3 "B"
            COLON@3..4 ":"
            EXPR_TERTIARY@4..9
              EXPR_VAR@4..5
                IDENT@4..5 "C"
              QUERY@5..6 "?"
              EXPR_VAR@6..7
                IDENT@6..7 "D"
              COLON@7..8 ":"
              EXPR_VAR@8..9
                IDENT@8..9 "E"
        "#);
    }

    #[test]
    fn then_tertiary() {
        let syntax = parse_expr_success("A ? B ? C : D : E");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..9
          EXPR_TERTIARY@0..9
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            QUERY@1..2 "?"
            EXPR_TERTIARY@2..7
              EXPR_VAR@2..3
                IDENT@2..3 "B"
              QUERY@3..4 "?"
              EXPR_VAR@4..5
                IDENT@4..5 "C"
              COLON@5..6 ":"
              EXPR_VAR@6..7
                IDENT@6..7 "D"
            COLON@7..8 ":"
            EXPR_VAR@8..9
              IDENT@8..9 "E"
        "#);
    }

    #[test]
    fn then_else_tertiary() {
        let syntax = parse_expr_success("A ? B ? C : D : E ? F : G");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..13
          EXPR_TERTIARY@0..13
            EXPR_VAR@0..1
              IDENT@0..1 "A"
            QUERY@1..2 "?"
            EXPR_TERTIARY@2..7
              EXPR_VAR@2..3
                IDENT@2..3 "B"
              QUERY@3..4 "?"
              EXPR_VAR@4..5
                IDENT@4..5 "C"
              COLON@5..6 ":"
              EXPR_VAR@6..7
                IDENT@6..7 "D"
            COLON@7..8 ":"
            EXPR_TERTIARY@8..13
              EXPR_VAR@8..9
                IDENT@8..9 "E"
              QUERY@9..10 "?"
              EXPR_VAR@10..11
                IDENT@10..11 "F"
              COLON@11..12 ":"
              EXPR_VAR@12..13
                IDENT@12..13 "G"
        "#);
    }
}

mod level_infix {
    use super::*;

    #[test]
    fn or_or() {
        let result = parse_expr("A || B || C");
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        assert_debug_snapshot!(result.syntax, @r#"
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
        let result = parse_expr("A || B || $");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..7
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
          ERROR@6..7
            UNKNOWN@6..7 "$"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:7-1:8: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LBRACE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
        "#);
    }

    #[test]
    fn add_add_err() {
        let result = parse_expr("A + B + $");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..5
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
          ERROR@4..5
            UNKNOWN@4..5 "$"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:5-1:6: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LBRACE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
        "#);
    }
}

mod level_prefix {
    use super::*;

    #[test]
    fn not() {
        let result = parse_expr("!A");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..2
          EXPR_PREFIX@0..2
            OP_PREFIX@0..1
              BANG@0..1 "!"
            EXPR_VAR@1..2
              IDENT@1..2 "A"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn not_not() {
        let result = parse_expr("!!A");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..3
          EXPR_PREFIX@0..3
            OP_PREFIX@0..1
              BANG@0..1 "!"
            EXPR_PREFIX@1..3
              OP_PREFIX@1..2
                BANG@1..2 "!"
              EXPR_VAR@2..3
                IDENT@2..3 "A"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @"");
    }

    #[test]
    fn not_not_err() {
        let result = parse_expr("!!$");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..3
          EXPR_PREFIX@0..2
            OP_PREFIX@0..1
              BANG@0..1 "!"
            EXPR_PREFIX@1..2
              OP_PREFIX@1..2
                BANG@1..2 "!"
          ERROR@2..3
            UNKNOWN@2..3 "$"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:3-1:4: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LBRACE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
        "#);
    }
}

mod expr_closure {
    use super::*;

    #[test]
    fn no_params() {
        let syntax = parse_expr_success("|| 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..3
          EXPR_CLOSURE@0..3
            PARAMS_CLOSURE@0..2
              BAR_BAR@0..2 "||"
            EXPR_LIT@2..3
              LIT_NAT@2..3 "1"
        "#);
    }

    #[test]
    fn one_param() {
        let syntax = parse_expr_success("|x| 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          EXPR_CLOSURE@0..4
            PARAMS_CLOSURE@0..3
              BAR@0..1 "|"
              BINDER@1..2
                IDENT@1..2 "x"
              BAR@2..3 "|"
            EXPR_LIT@3..4
              LIT_NAT@3..4 "1"
        "#);
    }

    #[test]
    fn two_params() {
        let syntax = parse_expr_success("|x, y| 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..6
          EXPR_CLOSURE@0..6
            PARAMS_CLOSURE@0..5
              BAR@0..1 "|"
              BINDER@1..2
                IDENT@1..2 "x"
              COMMA@2..3 ","
              BINDER@3..4
                IDENT@3..4 "y"
              BAR@4..5 "|"
            EXPR_LIT@5..6
              LIT_NAT@5..6 "1"
        "#);
    }

    #[test]
    fn body_atom() {
        let syntax = parse_expr_success("|x| 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          EXPR_CLOSURE@0..4
            PARAMS_CLOSURE@0..3
              BAR@0..1 "|"
              BINDER@1..2
                IDENT@1..2 "x"
              BAR@2..3 "|"
            EXPR_LIT@3..4
              LIT_NAT@3..4 "1"
        "#);
    }

    #[test]
    fn body_infix() {
        let syntax = parse_expr_success("|x| 1 + 2");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..6
          EXPR_CLOSURE@0..6
            PARAMS_CLOSURE@0..3
              BAR@0..1 "|"
              BINDER@1..2
                IDENT@1..2 "x"
              BAR@2..3 "|"
            EXPR_INFIX@3..6
              EXPR_LIT@3..4
                LIT_NAT@3..4 "1"
              OP_INFIX@4..5
                PLUS@4..5 "+"
              EXPR_LIT@5..6
                LIT_NAT@5..6 "2"
        "#);
    }

    #[test]
    fn body_block() {
        let syntax = parse_expr_success("|x| {}");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          EXPR_CLOSURE@0..5
            PARAMS_CLOSURE@0..3
              BAR@0..1 "|"
              BINDER@1..2
                IDENT@1..2 "x"
              BAR@2..3 "|"
            BLOCK@3..5
              LBRACE@3..4 "{"
              RBRACE@4..5 "}"
        "#);
    }
}
