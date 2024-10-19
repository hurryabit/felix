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
        if let Err(problem) = parser.expr(syntax::TokenKind::EOF.into()) {
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
        if let Err(problem) = parser.type_(syntax::TokenKind::EOF.into()) {
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
    let syntax = parse_success(
        r#"
        (* Slow recursive version of Fibonacci. *)
        let rec fib_slow = fun n ->
            if n <= 1 then
                0
            else
                fib (n-1) + fib (n-2)

        (* Faster tail-recursive version of Fibonacci. *)
        let fib_faster = fun n ->
            let rec go = fun (n, (a, b)) ->
                if n == 0 then
                    a
                else
                    go (n-1, (b, a+b))
            in
            go (n, (0, 1))
        "#,
    );
    assert_debug_snapshot!(syntax, @r#"
    PROGRAM@0..488
      WHITESPACE@0..9 "\n        "
      COMMENT@9..51 "(* Slow recursive ver ..."
      WHITESPACE@51..60 "\n        "
      DEFN_LET_REC@60..187
        KW_LET@60..63 "let"
        WHITESPACE@63..64 " "
        KW_REC@64..67 "rec"
        WHITESPACE@67..68 " "
        BIND_EXPR@68..187
          PAT_IDENT@68..76
            IDENT@68..76 "fib_slow"
          WHITESPACE@76..77 " "
          EQ@77..78 "="
          WHITESPACE@78..79 " "
          EXPR_FUN@79..187
            KW_FUN@79..82 "fun"
            WHITESPACE@82..83 " "
            PAT_IDENT@83..84
              IDENT@83..84 "n"
            WHITESPACE@84..85 " "
            ARROW@85..87 "->"
            WHITESPACE@87..100 "\n            "
            EXPR_IF@100..187
              KW_IF@100..102 "if"
              WHITESPACE@102..103 " "
              EXPR_INFIX@103..109
                EXPR_REF@103..104
                  IDENT@103..104 "n"
                WHITESPACE@104..105 " "
                OP_EXPR_INFIX@105..107
                  LT_EQ@105..107 "<="
                WHITESPACE@107..108 " "
                EXPR_LIT@108..109
                  LIT_NAT@108..109 "1"
              WHITESPACE@109..110 " "
              KW_THEN@110..114 "then"
              WHITESPACE@114..131 "\n                "
              EXPR_LIT@131..132
                LIT_NAT@131..132 "0"
              WHITESPACE@132..145 "\n            "
              KW_ELSE@145..149 "else"
              WHITESPACE@149..166 "\n                "
              EXPR_INFIX@166..187
                EXPR_APP@166..175
                  EXPR_REF@166..169
                    IDENT@166..169 "fib"
                  WHITESPACE@169..170 " "
                  EXPR_PAREN@170..175
                    LPAREN@170..171 "("
                    EXPR_INFIX@171..174
                      EXPR_REF@171..172
                        IDENT@171..172 "n"
                      OP_EXPR_INFIX@172..173
                        MINUS@172..173 "-"
                      EXPR_LIT@173..174
                        LIT_NAT@173..174 "1"
                    RPAREN@174..175 ")"
                WHITESPACE@175..176 " "
                OP_EXPR_INFIX@176..177
                  PLUS@176..177 "+"
                WHITESPACE@177..178 " "
                EXPR_APP@178..187
                  EXPR_REF@178..181
                    IDENT@178..181 "fib"
                  WHITESPACE@181..182 " "
                  EXPR_PAREN@182..187
                    LPAREN@182..183 "("
                    EXPR_INFIX@183..186
                      EXPR_REF@183..184
                        IDENT@183..184 "n"
                      OP_EXPR_INFIX@184..185
                        MINUS@184..185 "-"
                      EXPR_LIT@185..186
                        LIT_NAT@185..186 "2"
                    RPAREN@186..187 ")"
      WHITESPACE@187..197 "\n\n        "
      COMMENT@197..246 "(* Faster tail-recurs ..."
      WHITESPACE@246..255 "\n        "
      DEFN_LET@255..479
        KW_LET@255..258 "let"
        WHITESPACE@258..259 " "
        BIND_EXPR@259..479
          PAT_IDENT@259..269
            IDENT@259..269 "fib_faster"
          WHITESPACE@269..270 " "
          EQ@270..271 "="
          WHITESPACE@271..272 " "
          EXPR_FUN@272..479
            KW_FUN@272..275 "fun"
            WHITESPACE@275..276 " "
            PAT_IDENT@276..277
              IDENT@276..277 "n"
            WHITESPACE@277..278 " "
            ARROW@278..280 "->"
            WHITESPACE@280..293 "\n            "
            EXPR_LET_REC@293..479
              KW_LET@293..296 "let"
              WHITESPACE@296..297 " "
              KW_REC@297..300 "rec"
              WHITESPACE@300..301 " "
              BIND_EXPR@301..437
                PAT_IDENT@301..303
                  IDENT@301..303 "go"
                WHITESPACE@303..304 " "
                EQ@304..305 "="
                WHITESPACE@305..306 " "
                EXPR_FUN@306..437
                  KW_FUN@306..309 "fun"
                  WHITESPACE@309..310 " "
                  PAT_PAIR@310..321
                    LPAREN@310..311 "("
                    PAT_IDENT@311..312
                      IDENT@311..312 "n"
                    COMMA@312..313 ","
                    WHITESPACE@313..314 " "
                    PAT_PAIR@314..320
                      LPAREN@314..315 "("
                      PAT_IDENT@315..316
                        IDENT@315..316 "a"
                      COMMA@316..317 ","
                      WHITESPACE@317..318 " "
                      PAT_IDENT@318..319
                        IDENT@318..319 "b"
                      RPAREN@319..320 ")"
                    RPAREN@320..321 ")"
                  WHITESPACE@321..322 " "
                  ARROW@322..324 "->"
                  WHITESPACE@324..341 "\n                "
                  EXPR_IF@341..437
                    KW_IF@341..343 "if"
                    WHITESPACE@343..344 " "
                    EXPR_INFIX@344..350
                      EXPR_REF@344..345
                        IDENT@344..345 "n"
                      WHITESPACE@345..346 " "
                      OP_EXPR_INFIX@346..348
                        EQ_EQ@346..348 "=="
                      WHITESPACE@348..349 " "
                      EXPR_LIT@349..350
                        LIT_NAT@349..350 "0"
                    WHITESPACE@350..351 " "
                    KW_THEN@351..355 "then"
                    WHITESPACE@355..376 "\n                    "
                    EXPR_REF@376..377
                      IDENT@376..377 "a"
                    WHITESPACE@377..394 "\n                "
                    KW_ELSE@394..398 "else"
                    WHITESPACE@398..419 "\n                    "
                    EXPR_APP@419..437
                      EXPR_REF@419..421
                        IDENT@419..421 "go"
                      WHITESPACE@421..422 " "
                      EXPR_PAIR@422..437
                        LPAREN@422..423 "("
                        EXPR_INFIX@423..426
                          EXPR_REF@423..424
                            IDENT@423..424 "n"
                          OP_EXPR_INFIX@424..425
                            MINUS@424..425 "-"
                          EXPR_LIT@425..426
                            LIT_NAT@425..426 "1"
                        COMMA@426..427 ","
                        WHITESPACE@427..428 " "
                        EXPR_PAIR@428..436
                          LPAREN@428..429 "("
                          EXPR_REF@429..430
                            IDENT@429..430 "b"
                          COMMA@430..431 ","
                          WHITESPACE@431..432 " "
                          EXPR_INFIX@432..435
                            EXPR_REF@432..433
                              IDENT@432..433 "a"
                            OP_EXPR_INFIX@433..434
                              PLUS@433..434 "+"
                            EXPR_REF@434..435
                              IDENT@434..435 "b"
                          RPAREN@435..436 ")"
                        RPAREN@436..437 ")"
              WHITESPACE@437..450 "\n            "
              KW_IN@450..452 "in"
              WHITESPACE@452..465 "\n            "
              EXPR_APP@465..479
                EXPR_REF@465..467
                  IDENT@465..467 "go"
                WHITESPACE@467..468 " "
                EXPR_PAIR@468..479
                  LPAREN@468..469 "("
                  EXPR_REF@469..470
                    IDENT@469..470 "n"
                  COMMA@470..471 ","
                  WHITESPACE@471..472 " "
                  EXPR_PAIR@472..478
                    LPAREN@472..473 "("
                    EXPR_LIT@473..474
                      LIT_NAT@473..474 "0"
                    COMMA@474..475 ","
                    WHITESPACE@475..476 " "
                    EXPR_LIT@476..477
                      LIT_NAT@476..477 "1"
                    RPAREN@477..478 ")"
                  RPAREN@478..479 ")"
      WHITESPACE@479..488 "\n        "
    "#);
}

#[test]
fn empty() {
    let syntax = parse_success("");
    assert_debug_snapshot!(syntax, @r#"
    PROGRAM@0..0
    "#);
}

#[test]
fn one_good_fn() {
    let syntax = parse_success("let f = fun () -> 1");
    assert_debug_snapshot!(syntax, @r#"
    PROGRAM@0..19
      DEFN_LET@0..19
        KW_LET@0..3 "let"
        WHITESPACE@3..4 " "
        BIND_EXPR@4..19
          PAT_IDENT@4..5
            IDENT@4..5 "f"
          WHITESPACE@5..6 " "
          EQ@6..7 "="
          WHITESPACE@7..8 " "
          EXPR_FUN@8..19
            KW_FUN@8..11 "fun"
            WHITESPACE@11..12 " "
            PAT_UNIT@12..14
              LPAREN@12..13 "("
              RPAREN@13..14 ")"
            WHITESPACE@14..15 " "
            ARROW@15..17 "->"
            WHITESPACE@17..18 " "
            EXPR_LIT@18..19
              LIT_NAT@18..19 "1"
    "#);
}

#[test]
fn two_good_fns() {
    let syntax = parse_success("let f = fun () -> 1 let g = fun () -> 2");
    assert_debug_snapshot!(syntax, @r#"
    PROGRAM@0..39
      DEFN_LET@0..19
        KW_LET@0..3 "let"
        WHITESPACE@3..4 " "
        BIND_EXPR@4..19
          PAT_IDENT@4..5
            IDENT@4..5 "f"
          WHITESPACE@5..6 " "
          EQ@6..7 "="
          WHITESPACE@7..8 " "
          EXPR_FUN@8..19
            KW_FUN@8..11 "fun"
            WHITESPACE@11..12 " "
            PAT_UNIT@12..14
              LPAREN@12..13 "("
              RPAREN@13..14 ")"
            WHITESPACE@14..15 " "
            ARROW@15..17 "->"
            WHITESPACE@17..18 " "
            EXPR_LIT@18..19
              LIT_NAT@18..19 "1"
      WHITESPACE@19..20 " "
      DEFN_LET@20..39
        KW_LET@20..23 "let"
        WHITESPACE@23..24 " "
        BIND_EXPR@24..39
          PAT_IDENT@24..25
            IDENT@24..25 "g"
          WHITESPACE@25..26 " "
          EQ@26..27 "="
          WHITESPACE@27..28 " "
          EXPR_FUN@28..39
            KW_FUN@28..31 "fun"
            WHITESPACE@31..32 " "
            PAT_UNIT@32..34
              LPAREN@32..33 "("
              RPAREN@33..34 ")"
            WHITESPACE@34..35 " "
            ARROW@35..37 "->"
            WHITESPACE@37..38 " "
            EXPR_LIT@38..39
              LIT_NAT@38..39 "2"
    "#);
}

#[test]
fn one_good_fn_between_errors() {
    let result = parse(" $ let f = fun () -> 1 $ ");
    assert_debug_snapshot!(result.syntax, @r#"
    PROGRAM@0..25
      WHITESPACE@0..1 " "
      ERROR@1..2
        UNKNOWN@1..2 "$"
      WHITESPACE@2..3 " "
      DEFN_LET@3..22
        KW_LET@3..6 "let"
        WHITESPACE@6..7 " "
        BIND_EXPR@7..22
          PAT_IDENT@7..8
            IDENT@7..8 "f"
          WHITESPACE@8..9 " "
          EQ@9..10 "="
          WHITESPACE@10..11 " "
          EXPR_FUN@11..22
            KW_FUN@11..14 "fun"
            WHITESPACE@14..15 " "
            PAT_UNIT@15..17
              LPAREN@15..16 "("
              RPAREN@16..17 ")"
            WHITESPACE@17..18 " "
            ARROW@18..20 "->"
            WHITESPACE@20..21 " "
            EXPR_LIT@21..22
              LIT_NAT@21..22 "1"
      WHITESPACE@22..23 " "
      ERROR@23..24
        UNKNOWN@23..24 "$"
      WHITESPACE@24..25 " "
    "#);
    assert_snapshot!(dump_problems(&result.problems), @r#"
    ERROR 1:2-1:3: Found UNKNOWN, expected KW_LET | KW_TYPE. [parser/program]
    ERROR 1:24-1:25: Found UNKNOWN, expected KW_LET | KW_TYPE | AND | DIV | EQ_EQ | GT | GT_EQ | LT | LT_EQ | MINUS | MOD | NOT_EQ | OR | PLUS | TIMES | EOF. [parser/expr_fun]
    "#);
}

#[test]
fn infix() {
    let syntax = parse_success("let f = fun x -> x + x");
    assert_debug_snapshot!(syntax, @r#"
    PROGRAM@0..22
      DEFN_LET@0..22
        KW_LET@0..3 "let"
        WHITESPACE@3..4 " "
        BIND_EXPR@4..22
          PAT_IDENT@4..5
            IDENT@4..5 "f"
          WHITESPACE@5..6 " "
          EQ@6..7 "="
          WHITESPACE@7..8 " "
          EXPR_FUN@8..22
            KW_FUN@8..11 "fun"
            WHITESPACE@11..12 " "
            PAT_IDENT@12..13
              IDENT@12..13 "x"
            WHITESPACE@13..14 " "
            ARROW@14..16 "->"
            WHITESPACE@16..17 " "
            EXPR_INFIX@17..22
              EXPR_REF@17..18
                IDENT@17..18 "x"
              WHITESPACE@18..19 " "
              OP_EXPR_INFIX@19..20
                PLUS@19..20 "+"
              WHITESPACE@20..21 " "
              EXPR_REF@21..22
                IDENT@21..22 "x"
    "#);
}

#[test]
fn call() {
    let syntax = parse_success("let rec f = fun x -> f x");
    assert_debug_snapshot!(syntax, @r#"
    PROGRAM@0..24
      DEFN_LET_REC@0..24
        KW_LET@0..3 "let"
        WHITESPACE@3..4 " "
        KW_REC@4..7 "rec"
        WHITESPACE@7..8 " "
        BIND_EXPR@8..24
          PAT_IDENT@8..9
            IDENT@8..9 "f"
          WHITESPACE@9..10 " "
          EQ@10..11 "="
          WHITESPACE@11..12 " "
          EXPR_FUN@12..24
            KW_FUN@12..15 "fun"
            WHITESPACE@15..16 " "
            PAT_IDENT@16..17
              IDENT@16..17 "x"
            WHITESPACE@17..18 " "
            ARROW@18..20 "->"
            WHITESPACE@20..21 " "
            EXPR_APP@21..24
              EXPR_REF@21..22
                IDENT@21..22 "f"
              WHITESPACE@22..23 " "
              EXPR_REF@23..24
                IDENT@23..24 "x"
    "#);
}

mod type_ {
    use super::*;

    #[test]
    fn arrow_arrow() {
        let syntax = parse_type_success("A -> B -> C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..3
              ARROW@1..3 "->"
            TYPE_INFIX@3..7
              TYPE_REF@3..4
                IDENT@3..4 "B"
              OP_TYPE_INFIX@4..6
                ARROW@4..6 "->"
              TYPE_REF@6..7
                IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn arrow_union() {
        let syntax = parse_type_success(r"A -> B \/ C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..3
              ARROW@1..3 "->"
            TYPE_INFIX@3..7
              TYPE_REF@3..4
                IDENT@3..4 "B"
              OP_TYPE_INFIX@4..6
                UNION@4..6 "\\/"
              TYPE_REF@6..7
                IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn union_arrow() {
        let syntax = parse_type_success(r"A \/ B -> C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_INFIX@0..4
              TYPE_REF@0..1
                IDENT@0..1 "A"
              OP_TYPE_INFIX@1..3
                UNION@1..3 "\\/"
              TYPE_REF@3..4
                IDENT@3..4 "B"
            OP_TYPE_INFIX@4..6
              ARROW@4..6 "->"
            TYPE_REF@6..7
              IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn union_union() {
        let syntax = parse_type_success(r"A \/ B \/ C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..3
              UNION@1..3 "\\/"
            TYPE_INFIX@3..7
              TYPE_REF@3..4
                IDENT@3..4 "B"
              OP_TYPE_INFIX@4..6
                UNION@4..6 "\\/"
              TYPE_REF@6..7
                IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn union_inter() {
        let syntax = parse_type_success(r"A \/ B /\ C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..3
              UNION@1..3 "\\/"
            TYPE_INFIX@3..7
              TYPE_REF@3..4
                IDENT@3..4 "B"
              OP_TYPE_INFIX@4..6
                INTER@4..6 "/\\"
              TYPE_REF@6..7
                IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn inter_union() {
        let syntax = parse_type_success(r"A /\ B \/ C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_INFIX@0..4
              TYPE_REF@0..1
                IDENT@0..1 "A"
              OP_TYPE_INFIX@1..3
                INTER@1..3 "/\\"
              TYPE_REF@3..4
                IDENT@3..4 "B"
            OP_TYPE_INFIX@4..6
              UNION@4..6 "\\/"
            TYPE_REF@6..7
              IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn inter_inter() {
        let syntax = parse_type_success(r"A /\ B /\ C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..7
          TYPE_INFIX@0..7
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..3
              INTER@1..3 "/\\"
            TYPE_INFIX@3..7
              TYPE_REF@3..4
                IDENT@3..4 "B"
              OP_TYPE_INFIX@4..6
                INTER@4..6 "/\\"
              TYPE_REF@6..7
                IDENT@6..7 "C"
        "#);
    }

    #[test]
    fn inter_times() {
        let syntax = parse_type_success(r"A /\ B * C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..6
          TYPE_INFIX@0..6
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..3
              INTER@1..3 "/\\"
            TYPE_INFIX@3..6
              TYPE_REF@3..4
                IDENT@3..4 "B"
              OP_TYPE_INFIX@4..5
                TIMES@4..5 "*"
              TYPE_REF@5..6
                IDENT@5..6 "C"
        "#);
    }

    #[test]
    fn times_inter() {
        let syntax = parse_type_success(r"A * B /\ C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..6
          TYPE_INFIX@0..6
            TYPE_INFIX@0..3
              TYPE_REF@0..1
                IDENT@0..1 "A"
              OP_TYPE_INFIX@1..2
                TIMES@1..2 "*"
              TYPE_REF@2..3
                IDENT@2..3 "B"
            OP_TYPE_INFIX@3..5
              INTER@3..5 "/\\"
            TYPE_REF@5..6
              IDENT@5..6 "C"
        "#);
    }

    #[test]
    fn times_times() {
        let syntax = parse_type_success("A * B * C");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..5
          TYPE_INFIX@0..5
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..2
              TIMES@1..2 "*"
            TYPE_INFIX@2..5
              TYPE_REF@2..3
                IDENT@2..3 "B"
              OP_TYPE_INFIX@3..4
                TIMES@3..4 "*"
              TYPE_REF@4..5
                IDENT@4..5 "C"
        "#);
    }

    #[test]
    fn times_compl() {
        let syntax = parse_type_success(r"A * ~B");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          TYPE_INFIX@0..4
            TYPE_REF@0..1
              IDENT@0..1 "A"
            OP_TYPE_INFIX@1..2
              TIMES@1..2 "*"
            TYPE_PREFIX@2..4
              OP_TYPE_PREFIX@2..3
                COMPL@2..3 "~"
              TYPE_REF@3..4
                IDENT@3..4 "B"
        "#);
    }

    #[test]
    fn compl_times() {
        let syntax = parse_type_success("~A * B");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          TYPE_INFIX@0..4
            TYPE_PREFIX@0..2
              OP_TYPE_PREFIX@0..1
                COMPL@0..1 "~"
              TYPE_REF@1..2
                IDENT@1..2 "A"
            OP_TYPE_INFIX@2..3
              TIMES@2..3 "*"
            TYPE_REF@3..4
              IDENT@3..4 "B"
        "#);
    }

    #[test]
    fn compl_compl() {
        let syntax = parse_type_success("~~A");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..3
          TYPE_PREFIX@0..3
            OP_TYPE_PREFIX@0..1
              COMPL@0..1 "~"
            TYPE_PREFIX@1..3
              OP_TYPE_PREFIX@1..2
                COMPL@1..2 "~"
              TYPE_REF@2..3
                IDENT@2..3 "A"
        "#);
    }

    #[test]
    fn unit() {
        let syntax = parse_type_success("Unit");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..4
          TYPE_BUILTIN@0..4
            TY_UNIT@0..4 "Unit"
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

mod level_infix {
    use super::*;

    #[test]
    fn or_or() {
        let result = parse_expr("A || B || C");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..7
          EXPR_INFIX@0..7
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              OR@1..3 "||"
            EXPR_INFIX@3..7
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..6
                OR@4..6 "||"
              EXPR_REF@6..7
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
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              OR@1..3 "||"
            EXPR_INFIX@3..7
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..6
                AND@4..6 "&&"
              EXPR_REF@6..7
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..3
                AND@1..3 "&&"
              EXPR_REF@3..4
                IDENT@3..4 "B"
            OP_EXPR_INFIX@4..6
              OR@4..6 "||"
            EXPR_REF@6..7
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
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              AND@1..3 "&&"
            EXPR_INFIX@3..7
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..6
                AND@4..6 "&&"
              EXPR_REF@6..7
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
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              AND@1..3 "&&"
            EXPR_INFIX@3..7
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..6
                EQ_EQ@4..6 "=="
              EXPR_REF@6..7
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..3
                EQ_EQ@1..3 "=="
              EXPR_REF@3..4
                IDENT@3..4 "B"
            OP_EXPR_INFIX@4..6
              AND@4..6 "&&"
            EXPR_REF@6..7
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..3
                EQ_EQ@1..3 "=="
              EXPR_REF@3..4
                IDENT@3..4 "B"
            ERROR@4..6
              NOT_EQ@4..6 "!="
            EXPR_REF@6..7
              IDENT@6..7 "C"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:5-1:7: Cannot chain operators EQ_EQ and NOT_EQ [parser/program]
        "#);
    }

    #[test]
    fn cmp_add() {
        let result = parse_expr("A == B + C");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..6
          EXPR_INFIX@0..6
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              EQ_EQ@1..3 "=="
            EXPR_INFIX@3..6
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..5
                PLUS@4..5 "+"
              EXPR_REF@5..6
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..2
                PLUS@1..2 "+"
              EXPR_REF@2..3
                IDENT@2..3 "B"
            OP_EXPR_INFIX@3..5
              EQ_EQ@3..5 "=="
            EXPR_REF@5..6
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..2
                PLUS@1..2 "+"
              EXPR_REF@2..3
                IDENT@2..3 "B"
            OP_EXPR_INFIX@3..4
              MINUS@3..4 "-"
            EXPR_REF@4..5
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
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..2
              PLUS@1..2 "+"
            EXPR_INFIX@2..5
              EXPR_REF@2..3
                IDENT@2..3 "B"
              OP_EXPR_INFIX@3..4
                TIMES@3..4 "*"
              EXPR_REF@4..5
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..2
                TIMES@1..2 "*"
              EXPR_REF@2..3
                IDENT@2..3 "B"
            OP_EXPR_INFIX@3..4
              PLUS@3..4 "+"
            EXPR_REF@4..5
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
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..2
                TIMES@1..2 "*"
              EXPR_REF@2..3
                IDENT@2..3 "B"
            OP_EXPR_INFIX@3..4
              DIV@3..4 "/"
            EXPR_REF@4..5
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
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              OR@1..3 "||"
            EXPR_INFIX@3..10
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..6
                OR@4..6 "||"
              EXPR_INFIX@6..10
                EXPR_REF@6..7
                  IDENT@6..7 "C"
                OP_EXPR_INFIX@7..9
                  OR@7..9 "||"
                EXPR_REF@9..10
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
                EXPR_REF@0..1
                  IDENT@0..1 "A"
                OP_EXPR_INFIX@1..2
                  LT@1..2 "<"
                EXPR_REF@2..3
                  IDENT@2..3 "B"
              ERROR@3..5
                EQ_EQ@3..5 "=="
              EXPR_REF@5..6
                IDENT@5..6 "C"
            ERROR@6..7
              GT@6..7 ">"
            EXPR_REF@7..8
              IDENT@7..8 "D"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:4-1:6: Cannot chain operators LT and EQ_EQ [parser/program]
        ERROR 1:7-1:8: Cannot chain operators EQ_EQ and GT [parser/program]
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
                EXPR_REF@0..1
                  IDENT@0..1 "A"
                OP_EXPR_INFIX@1..2
                  PLUS@1..2 "+"
                EXPR_REF@2..3
                  IDENT@2..3 "B"
              OP_EXPR_INFIX@3..4
                PLUS@3..4 "+"
              EXPR_REF@4..5
                IDENT@4..5 "C"
            OP_EXPR_INFIX@5..6
              PLUS@5..6 "+"
            EXPR_REF@6..7
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
            EXPR_REF@0..1
              IDENT@0..1 "A"
            OP_EXPR_INFIX@1..3
              OR@1..3 "||"
            EXPR_INFIX@3..6
              EXPR_REF@3..4
                IDENT@3..4 "B"
              OP_EXPR_INFIX@4..6
                OR@4..6 "||"
          ERROR@6..7
            UNKNOWN@6..7 "$"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:7-1:8: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LPAREN | NOT | IDENT | LIT_NAT. [parser/program]
        "#);
    }

    #[test]
    fn add_add_err() {
        let result = parse_expr("A + B + $");
        assert_debug_snapshot!(result.syntax, @r#"
        PROGRAM@0..5
          EXPR_INFIX@0..4
            EXPR_INFIX@0..3
              EXPR_REF@0..1
                IDENT@0..1 "A"
              OP_EXPR_INFIX@1..2
                PLUS@1..2 "+"
              EXPR_REF@2..3
                IDENT@2..3 "B"
            OP_EXPR_INFIX@3..4
              PLUS@3..4 "+"
          ERROR@4..5
            UNKNOWN@4..5 "$"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:5-1:6: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LPAREN | NOT | IDENT | LIT_NAT. [parser/program]
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
            OP_EXPR_PREFIX@0..1
              NOT@0..1 "!"
            EXPR_REF@1..2
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
            OP_EXPR_PREFIX@0..1
              NOT@0..1 "!"
            EXPR_PREFIX@1..3
              OP_EXPR_PREFIX@1..2
                NOT@1..2 "!"
              EXPR_REF@2..3
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
            OP_EXPR_PREFIX@0..1
              NOT@0..1 "!"
            EXPR_PREFIX@1..2
              OP_EXPR_PREFIX@1..2
                NOT@1..2 "!"
          ERROR@2..3
            UNKNOWN@2..3 "$"
        "#);
        assert_snapshot!(dump_problems(&result.problems), @r#"
        ERROR 1:3-1:4: Found UNKNOWN, expected KW_FALSE | KW_TRUE | LPAREN | NOT | IDENT | LIT_NAT. [parser/program]
        "#);
    }
}

mod expr_fun {
    use super::*;

    #[test]
    fn no_params() {
        let syntax = parse_expr_success("fun () -> 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..8
          EXPR_FUN@0..8
            KW_FUN@0..3 "fun"
            PAT_UNIT@3..5
              LPAREN@3..4 "("
              RPAREN@4..5 ")"
            ARROW@5..7 "->"
            EXPR_LIT@7..8
              LIT_NAT@7..8 "1"
        "#);
    }

    #[test]
    fn one_param() {
        let syntax = parse_expr_success("fun x -> 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..9
          EXPR_FUN@0..9
            KW_FUN@0..3 "fun"
            WHITESPACE@3..5 "\u{a0}"
            PAT_IDENT@5..6
              IDENT@5..6 "x"
            ARROW@6..8 "->"
            EXPR_LIT@8..9
              LIT_NAT@8..9 "1"
        "#);
    }

    #[test]
    fn two_params() {
        let syntax = parse_expr_success("fun (x, y) -> 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..11
          EXPR_FUN@0..11
            KW_FUN@0..3 "fun"
            PAT_PAIR@3..8
              LPAREN@3..4 "("
              PAT_IDENT@4..5
                IDENT@4..5 "x"
              COMMA@5..6 ","
              PAT_IDENT@6..7
                IDENT@6..7 "y"
              RPAREN@7..8 ")"
            ARROW@8..10 "->"
            EXPR_LIT@10..11
              LIT_NAT@10..11 "1"
        "#);
    }

    #[test]
    fn body_atom() {
        let syntax = parse_expr_success("fun () -> 1");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..8
          EXPR_FUN@0..8
            KW_FUN@0..3 "fun"
            PAT_UNIT@3..5
              LPAREN@3..4 "("
              RPAREN@4..5 ")"
            ARROW@5..7 "->"
            EXPR_LIT@7..8
              LIT_NAT@7..8 "1"
        "#);
    }

    #[test]
    fn body_infix() {
        let syntax = parse_expr_success("fun () -> 1 + 2");
        assert_debug_snapshot!(syntax, @r#"
        PROGRAM@0..10
          EXPR_FUN@0..10
            KW_FUN@0..3 "fun"
            PAT_UNIT@3..5
              LPAREN@3..4 "("
              RPAREN@4..5 ")"
            ARROW@5..7 "->"
            EXPR_INFIX@7..10
              EXPR_LIT@7..8
                LIT_NAT@7..8 "1"
              OP_EXPR_INFIX@8..9
                PLUS@8..9 "+"
              EXPR_LIT@9..10
                LIT_NAT@9..10 "2"
        "#);
    }
}
