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
        const tag = lambdaParser.token(stream, state) ?? "whitespace";
        let current = stream.current();
        if (tag == "whitespace") current = encodeURI(current);
        tokens.push(`${tag}@${stream.start}..${stream.pos} :: ${current}`);
    }
    return tokens;
}

describe("tokenizer", () => {
    test("sample", () => {
        expect(tokenizer(SAMPLE)).toMatchInlineSnapshot(`
          [
            "comment@0..30 :: # Simple higher order function",
            "whitespace@30..31 :: %0A",
            "definitionKeyword@31..34 :: let",
            "whitespace@34..35 :: %20",
            "variableName@35..40 :: twice",
            "whitespace@40..41 :: %20",
            "punctuation@41..42 :: =",
            "whitespace@42..47 :: %0A%20%20%20%20",
            "keyword@47..48 :: λ",
            "variableName@48..49 :: f",
            "punctuation@49..50 :: :",
            "typeName@50..54 :: Unit",
            "whitespace@54..55 :: %20",
            "typeOperator@55..57 :: ->",
            "whitespace@57..58 :: %20",
            "typeName@58..62 :: Unit",
            "punctuation@62..63 :: .",
            "whitespace@63..64 :: %20",
            "keyword@64..65 :: λ",
            "variableName@65..66 :: u",
            "punctuation@66..67 :: :",
            "typeName@67..71 :: Unit",
            "punctuation@71..72 :: .",
            "whitespace@72..73 :: %20",
            "variableName@73..74 :: f",
            "whitespace@74..75 :: %20",
            "paren@75..76 :: (",
            "variableName@76..77 :: f",
            "whitespace@77..78 :: %20",
            "variableName@78..79 :: u",
            "paren@79..80 :: )",
            "whitespace@80..81 :: %0A",
            "definitionKeyword@81..83 :: in",
            "whitespace@83..84 :: %0A",
            "variableName@84..89 :: twice",
            "whitespace@89..90 :: %20",
            "paren@90..91 :: (",
            "keyword@91..92 :: λ",
            "variableName@92..93 :: u",
            "punctuation@93..94 :: :",
            "typeName@94..98 :: Unit",
            "punctuation@98..99 :: .",
            "whitespace@99..100 :: %20",
            "variableName@100..101 :: u",
            "paren@101..102 :: )",
            "whitespace@102..103 :: %20",
            "atom@103..107 :: unit",
            "whitespace@107..108 :: %0A",
          ]
        `);
    });
});
