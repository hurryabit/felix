import { StringStream } from "@codemirror/language";
import { describe, expect, test } from "vitest";
import { forTestsOnly, SAMPLE } from "./lambda";

const { lambdaParser } = forTestsOnly;

function tokenizer(input: string): string[] {
    const stream = new StringStream(input, 1, 1);
    const state = lambdaParser.startState!(1);
    const tokens = [] as string[];
    while (!stream.eol()) {
        stream.start = stream.pos;
        let kind = lambdaParser.token(stream, state);
        let current = stream.current();
        if (kind == null) {
            kind = "WHITESPACE";
            current = encodeURI(current);
        }
        tokens.push(`${kind}@${stream.start}..${stream.pos} :: ${current}`);
    }
    return tokens;
}

describe("tokenizer", () => {
    test("sample", () => {
        expect(tokenizer(SAMPLE)).toMatchInlineSnapshot(`
          [
            "COMMENT@0..30 :: # Simple higher order function",
            "WHITESPACE@30..31 :: %0A",
            "KW_LET@31..34 :: let",
            "WHITESPACE@34..35 :: %20",
            "ID_EXPR@35..40 :: twice",
            "WHITESPACE@40..41 :: %20",
            "EQUALS@41..42 :: =",
            "WHITESPACE@42..47 :: %0A%20%20%20%20",
            "GR_LAM_LOW@47..48 :: λ",
            "ID_EXPR@48..49 :: f",
            "COLON@49..50 :: :",
            "TY_UNIT@50..54 :: Unit",
            "WHITESPACE@54..55 :: %20",
            "OP_ARROW@55..57 :: ->",
            "WHITESPACE@57..58 :: %20",
            "TY_UNIT@58..62 :: Unit",
            "DOT@62..63 :: .",
            "WHITESPACE@63..64 :: %20",
            "GR_LAM_LOW@64..65 :: λ",
            "ID_EXPR@65..66 :: u",
            "COLON@66..67 :: :",
            "TY_UNIT@67..71 :: Unit",
            "DOT@71..72 :: .",
            "WHITESPACE@72..73 :: %20",
            "ID_EXPR@73..74 :: f",
            "WHITESPACE@74..75 :: %20",
            "LPAREN@75..76 :: (",
            "ID_EXPR@76..77 :: f",
            "WHITESPACE@77..78 :: %20",
            "ID_EXPR@78..79 :: u",
            "RPAREN@79..80 :: )",
            "WHITESPACE@80..81 :: %0A",
            "KW_IN@81..83 :: in",
            "WHITESPACE@83..84 :: %0A",
            "ID_EXPR@84..89 :: twice",
            "WHITESPACE@89..90 :: %20",
            "LPAREN@90..91 :: (",
            "GR_LAM_LOW@91..92 :: λ",
            "ID_EXPR@92..93 :: u",
            "COLON@93..94 :: :",
            "TY_UNIT@94..98 :: Unit",
            "DOT@98..99 :: .",
            "WHITESPACE@99..100 :: %20",
            "ID_EXPR@100..101 :: u",
            "RPAREN@101..102 :: )",
            "WHITESPACE@102..103 :: %20",
            "LIT_UNIT@103..107 :: unit",
            "WHITESPACE@107..108 :: %0A",
          ]
        `);
    });
});
