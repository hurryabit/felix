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

fn parse_type(input: &str) -> ParseResult {
    fn type_(parser: &mut Parser) {
        let mut parser = parser.with_root(syntax::NodeKind::PROGRAM);
        if let Err(problem) = parser.type_(syntax::TokenKind::EOF) {
            parser.push_problem(problem);
        }
        parser.skip_until(syntax::TokenKind::EOF);
    }

    let input = input.replace(" ", "");
    let mapper = Mapper::new(&input);
    let parser = Parser::new(&input, &mapper);
    parser.run(type_)
}

fn parse_type_success(input: &str) -> syntax::Node {
    let result = parse_type(input);
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
        let rec go = fn(n) {
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
    PROGRAM@0..506
      WHITESPACE@0..5 "\n    "
      COMMENT@5..40 "// Recursive version  ..."
      WHITESPACE@40..44 "    "
      DEFN_FN@44..166
        KW_FN@44..46 "fn"
        WHITESPACE@46..47 " "
        IDENT@47..54 "fib_rec"
        PARAMS@54..57
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
      DEFN_FN@226..501
        KW_FN@226..228 "fn"
        WHITESPACE@228..229 " "
        IDENT@229..240 "fib_tailrec"
        PARAMS@240..243
          LPAREN@240..241 "("
          BINDER@241..242
            IDENT@241..242 "n"
          RPAREN@242..243 ")"
        WHITESPACE@243..244 " "
        BLOCK@244..501
          LBRACE@244..245 "{"
          WHITESPACE@245..254 "\n        "
          STMT_LET@254..268
            KW_LET@254..257 "let"
            WHITESPACE@257..258 " "
            BINDING@258..267
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
            BINDING@281..290
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
          STMT_LET_REC@300..470
            KW_LET@300..303 "let"
            WHITESPACE@303..304 " "
            KW_REC@304..307 "rec"
            WHITESPACE@307..308 " "
            BINDING@308..469
              BINDER@308..310
                IDENT@308..310 "go"
              WHITESPACE@310..311 " "
              EQUALS@311..312 "="
              WHITESPACE@312..313 " "
              EXPR_FN@313..469
                KW_FN@313..315 "fn"
                PARAMS@315..318
                  LPAREN@315..316 "("
                  BINDER@316..317
                    IDENT@316..317 "n"
                  RPAREN@317..318 ")"
                WHITESPACE@318..319 " "
                BLOCK@319..469
                  LBRACE@319..320 "{"
                  WHITESPACE@320..333 "\n            "
                  STMT_IF@333..459
                    KW_IF@333..335 "if"
                    WHITESPACE@335..336 " "
                    EXPR_INFIX@336..341
                      EXPR_VAR@336..337
                        IDENT@336..337 "n"
                      WHITESPACE@337..338 " "
                      OP_INFIX@338..339
                        RANGLE@338..339 ">"
                      WHITESPACE@339..340 " "
                      EXPR_LIT@340..341
                        LIT_NAT@340..341 "0"
                    WHITESPACE@341..342 " "
                    BLOCK@342..459
                      LBRACE@342..343 "{"
                      WHITESPACE@343..360 "\n                "
                      STMT_LET@360..370
                        KW_LET@360..363 "let"
                        WHITESPACE@363..364 " "
                        BINDING@364..369
                          BINDER@364..365
                            IDENT@364..365 "c"
                          WHITESPACE@365..366 " "
                          EQUALS@366..367 "="
                          WHITESPACE@367..368 " "
                          EXPR_VAR@368..369
                            IDENT@368..369 "b"
                        SEMI@369..370 ";"
                      WHITESPACE@370..387 "\n                "
                      STMT_ASSIGN@387..397
                        EXPR_VAR@387..388
                          IDENT@387..388 "b"
                        WHITESPACE@388..389 " "
                        EQUALS@389..390 "="
                        WHITESPACE@390..391 " "
                        EXPR_INFIX@391..396
                          EXPR_VAR@391..392
                            IDENT@391..392 "a"
                          WHITESPACE@392..393 " "
                          OP_INFIX@393..394
                            PLUS@393..394 "+"
                          WHITESPACE@394..395 " "
                          EXPR_VAR@395..396
                            IDENT@395..396 "b"
                        SEMI@396..397 ";"
                      WHITESPACE@397..414 "\n                "
                      STMT_ASSIGN@414..420
                        EXPR_VAR@414..415
                          IDENT@414..415 "a"
                        WHITESPACE@415..416 " "
                        EQUALS@416..417 "="
                        WHITESPACE@417..418 " "
                        EXPR_VAR@418..419
                          IDENT@418..419 "c"
                        SEMI@419..420 ";"
                      WHITESPACE@420..437 "\n                "
                      STMT_EXPR@437..445
                        EXPR_CALL@437..444
                          EXPR_VAR@437..439
                            IDENT@437..439 "go"
                          ARGS@439..444
                            LPAREN@439..440 "("
                            EXPR_INFIX@440..443
                              EXPR_VAR@440..441
                                IDENT@440..441 "n"
                              OP_INFIX@441..442
                                MINUS@441..442 "-"
                              EXPR_LIT@442..443
                                LIT_NAT@442..443 "1"
                            RPAREN@443..444 ")"
                        SEMI@444..445 ";"
                      WHITESPACE@445..458 "\n            "
                      RBRACE@458..459 "}"
                  WHITESPACE@459..468 "\n        "
                  RBRACE@468..469 "}"
            SEMI@469..470 ";"
          WHITESPACE@470..479 "\n        "
          STMT_EXPR@479..485
            EXPR_CALL@479..484
              EXPR_VAR@479..481
                IDENT@479..481 "go"
              ARGS@481..484
                LPAREN@481..482 "("
                EXPR_VAR@482..483
                  IDENT@482..483 "n"
                RPAREN@483..484 ")"
            SEMI@484..485 ";"
          WHITESPACE@485..494 "\n        "
          EXPR_VAR@494..495
            IDENT@494..495 "a"
          WHITESPACE@495..500 "\n    "
          RBRACE@500..501 "}"
      WHITESPACE@501..506 "\n    "
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
        PARAMS@4..6
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
        PARAMS@4..6
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
        PARAMS@14..16
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
        PARAMS@7..9
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
    ERROR 1:2-1:3: Found UNKNOWN, expected KW_FN | KW_TYPE. [parser/program]
    ERROR 1:14-1:15: Found UNKNOWN, expected KW_FN | KW_TYPE. [parser/program]
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
        PARAMS@4..7
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
        PARAMS@4..7
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
        PARAMS@4..7
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
        PARAMS@4..6
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
        PARAMS@4..6
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

mod type_ {
    use super::*;

    #[test]
    fn fn_fn() {
        let syntax = parse_type_success("fn(A) -> fn(B) -> C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..15
          TYPE_FN@0..15
            KW_FN@0..2 "fn"
            LPAREN@2..3 "("
            LIST_TYPES@3..4
              TYPE_REF@3..4
                IDENT@3..4 "A"
            RPAREN@4..5 ")"
            MINUS_RANGLE@5..7 "->"
            TYPE_FN@7..15
              KW_FN@7..9 "fn"
              LPAREN@9..10 "("
              LIST_TYPES@10..11
                TYPE_REF@10..11
                  IDENT@10..11 "B"
              RPAREN@11..12 ")"
              MINUS_RANGLE@12..14 "->"
              TYPE_REF@14..15
                IDENT@14..15 "C"
        "#);
    }

    #[test]
    fn fn_union() {
        let syntax = parse_type_success("fn(A) -> B | C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..10
          TYPE_FN@0..10
            KW_FN@0..2 "fn"
            LPAREN@2..3 "("
            LIST_TYPES@3..4
              TYPE_REF@3..4
                IDENT@3..4 "A"
            RPAREN@4..5 ")"
            MINUS_RANGLE@5..7 "->"
            TYPE_UNION@7..10
              TYPE_REF@7..8
                IDENT@7..8 "B"
              BAR@8..9 "|"
              TYPE_REF@9..10
                IDENT@9..10 "C"
        "#);
    }

    #[test]
    fn union_fn_bad() {
        let result = parse_type("A | fn(B) -> C");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..10
          TYPE_UNION@0..2
            TYPE_REF@0..1
              IDENT@0..1 "A"
            BAR@1..2 "|"
          ERROR@2..10
            KW_FN@2..4 "fn"
            LPAREN@4..5 "("
            IDENT@5..6 "B"
            RPAREN@6..7 ")"
            MINUS_RANGLE@7..9 "->"
            IDENT@9..10 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:3-1:5: Found KW_FN, expected KW_ANY | KW_BOOL | KW_INT | KW_NEVER | LPAREN | TILDE | IDENT. [parser/type_union]
        "#)
    }

    #[test]
    fn union_fn() {
        let syntax = parse_type_success("A | (fn(B) -> C)");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..12
          TYPE_UNION@0..12
            TYPE_REF@0..1
              IDENT@0..1 "A"
            BAR@1..2 "|"
            TYPE_PAREN@2..12
              LPAREN@2..3 "("
              TYPE_FN@3..11
                KW_FN@3..5 "fn"
                LPAREN@5..6 "("
                LIST_TYPES@6..7
                  TYPE_REF@6..7
                    IDENT@6..7 "B"
                RPAREN@7..8 ")"
                MINUS_RANGLE@8..10 "->"
                TYPE_REF@10..11
                  IDENT@10..11 "C"
              RPAREN@11..12 ")"
        "#);
    }

    #[test]
    fn union_union() {
        let syntax = parse_type_success("A | B | C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          TYPE_UNION@0..5
            TYPE_REF@0..1
              IDENT@0..1 "A"
            BAR@1..2 "|"
            TYPE_UNION@2..5
              TYPE_REF@2..3
                IDENT@2..3 "B"
              BAR@3..4 "|"
              TYPE_REF@4..5
                IDENT@4..5 "C"
        "#);
    }

    #[test]
    fn union_intersection() {
        let syntax = parse_type_success("A | B & C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          TYPE_UNION@0..5
            TYPE_REF@0..1
              IDENT@0..1 "A"
            BAR@1..2 "|"
            TYPE_INTERSECTION@2..5
              TYPE_REF@2..3
                IDENT@2..3 "B"
              AMPER@3..4 "&"
              TYPE_REF@4..5
                IDENT@4..5 "C"
        "#);
    }

    #[test]
    fn intersection_union() {
        let syntax = parse_type_success("A & B | C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          TYPE_UNION@0..5
            TYPE_INTERSECTION@0..3
              TYPE_REF@0..1
                IDENT@0..1 "A"
              AMPER@1..2 "&"
              TYPE_REF@2..3
                IDENT@2..3 "B"
            BAR@3..4 "|"
            TYPE_REF@4..5
              IDENT@4..5 "C"
        "#);
    }

    #[test]
    fn intersection_intersection() {
        let syntax = parse_type_success("A & B & C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          TYPE_INTERSECTION@0..5
            TYPE_REF@0..1
              IDENT@0..1 "A"
            AMPER@1..2 "&"
            TYPE_INTERSECTION@2..5
              TYPE_REF@2..3
                IDENT@2..3 "B"
              AMPER@3..4 "&"
              TYPE_REF@4..5
                IDENT@4..5 "C"
        "#);
    }

    #[test]
    fn intersection_complement() {
        let syntax = parse_type_success("A & ~B");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          TYPE_INTERSECTION@0..4
            TYPE_REF@0..1
              IDENT@0..1 "A"
            AMPER@1..2 "&"
            TYPE_COMPLEMENT@2..4
              TILDE@2..3 "~"
              TYPE_REF@3..4
                IDENT@3..4 "B"
        "#);
    }

    #[test]
    fn complement_intersection() {
        let syntax = parse_type_success("~A & B");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          TYPE_INTERSECTION@0..4
            TYPE_COMPLEMENT@0..2
              TILDE@0..1 "~"
              TYPE_REF@1..2
                IDENT@1..2 "A"
            AMPER@2..3 "&"
            TYPE_REF@3..4
              IDENT@3..4 "B"
        "#);
    }

    #[test]
    fn complement_comlement() {
        let syntax = parse_type_success("~~A");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..3
          TYPE_COMPLEMENT@0..3
            TILDE@0..1 "~"
            TYPE_COMPLEMENT@1..3
              TILDE@1..2 "~"
              TYPE_REF@2..3
                IDENT@2..3 "A"
        "#);
    }

    #[test]
    fn tuple_zero() {
        let syntax = parse_type_success("()");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..2
          TYPE_TUPLE@0..2
            LPAREN@0..1 "("
            RPAREN@1..2 ")"
        "#);
    }

    #[test]
    fn tuple_one() {
        let syntax = parse_type_success("(A,)");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          TYPE_TUPLE@0..4
            LPAREN@0..1 "("
            TYPE_REF@1..2
              IDENT@1..2 "A"
            COMMA@2..3 ","
            RPAREN@3..4 ")"
        "#);
    }

    #[test]
    fn tuple_two() {
        let syntax = parse_type_success("(A, B)");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          TYPE_TUPLE@0..5
            LPAREN@0..1 "("
            TYPE_REF@1..2
              IDENT@1..2 "A"
            COMMA@2..3 ","
            TYPE_REF@3..4
              IDENT@3..4 "B"
            RPAREN@4..5 ")"
        "#);
    }

    #[test]
    fn paren() {
        let syntax = parse_type_success("(A)");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..3
          TYPE_PAREN@0..3
            LPAREN@0..1 "("
            TYPE_REF@1..2
              IDENT@1..2 "A"
            RPAREN@2..3 ")"
        "#);
    }
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
            PARAMS@4..6
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
            PARAMS@4..7
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
            PARAMS@4..10
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
        ERROR 1:7-1:8: Found UNKNOWN, expected KW_FALSE | KW_FN | KW_TRUE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
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
        ERROR 1:5-1:6: Found UNKNOWN, expected KW_FALSE | KW_FN | KW_TRUE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
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
        ERROR 1:3-1:4: Found UNKNOWN, expected KW_FALSE | KW_FN | KW_TRUE | LPAREN | BANG | IDENT | LIT_NAT. [parser/program]
        "#);
    }
}

mod expr_fn {
    use super::*;

    #[test]
    fn no_params() {
        let syntax = parse_expr_success("fn() { 1 }");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          EXPR_FN@0..7
            KW_FN@0..2 "fn"
            PARAMS@2..4
              LPAREN@2..3 "("
              RPAREN@3..4 ")"
            BLOCK@4..7
              LBRACE@4..5 "{"
              EXPR_LIT@5..6
                LIT_NAT@5..6 "1"
              RBRACE@6..7 "}"
        "#);
    }

    #[test]
    fn one_param() {
        let syntax = parse_expr_success("fn(x) { 1 }");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..8
          EXPR_FN@0..8
            KW_FN@0..2 "fn"
            PARAMS@2..5
              LPAREN@2..3 "("
              BINDER@3..4
                IDENT@3..4 "x"
              RPAREN@4..5 ")"
            BLOCK@5..8
              LBRACE@5..6 "{"
              EXPR_LIT@6..7
                LIT_NAT@6..7 "1"
              RBRACE@7..8 "}"
        "#);
    }

    #[test]
    fn two_params() {
        let syntax = parse_expr_success("fn(x, y) { 1 }");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..10
          EXPR_FN@0..10
            KW_FN@0..2 "fn"
            PARAMS@2..7
              LPAREN@2..3 "("
              BINDER@3..4
                IDENT@3..4 "x"
              COMMA@4..5 ","
              BINDER@5..6
                IDENT@5..6 "y"
              RPAREN@6..7 ")"
            BLOCK@7..10
              LBRACE@7..8 "{"
              EXPR_LIT@8..9
                LIT_NAT@8..9 "1"
              RBRACE@9..10 "}"
        "#);
    }

    #[test]
    fn body_atom() {
        let syntax = parse_expr_success("fn(x) { 1 }");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..8
          EXPR_FN@0..8
            KW_FN@0..2 "fn"
            PARAMS@2..5
              LPAREN@2..3 "("
              BINDER@3..4
                IDENT@3..4 "x"
              RPAREN@4..5 ")"
            BLOCK@5..8
              LBRACE@5..6 "{"
              EXPR_LIT@6..7
                LIT_NAT@6..7 "1"
              RBRACE@7..8 "}"
        "#);
    }

    #[test]
    fn body_infix() {
        let syntax = parse_expr_success("fn(x) { 1 + 2 }");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..10
          EXPR_FN@0..10
            KW_FN@0..2 "fn"
            PARAMS@2..5
              LPAREN@2..3 "("
              BINDER@3..4
                IDENT@3..4 "x"
              RPAREN@4..5 ")"
            BLOCK@5..10
              LBRACE@5..6 "{"
              EXPR_INFIX@6..9
                EXPR_LIT@6..7
                  LIT_NAT@6..7 "1"
                OP_INFIX@7..8
                  PLUS@7..8 "+"
                EXPR_LIT@8..9
                  LIT_NAT@8..9 "2"
              RBRACE@9..10 "}"
        "#);
    }

    #[test]
    fn body_block() {
        let syntax = parse_expr_success("fn(x) {}");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          EXPR_FN@0..7
            KW_FN@0..2 "fn"
            PARAMS@2..5
              LPAREN@2..3 "("
              BINDER@3..4
                IDENT@3..4 "x"
              RPAREN@4..5 ")"
            BLOCK@5..7
              LBRACE@5..6 "{"
              RBRACE@6..7 "}"
        "#);
    }
}
