// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import "ace-builds/src-noconflict/mode-text";

export class LambdaHighlightRules extends window.ace.acequire("ace/mode/text_highlight_rules")
    .TextHighlightRules {
    constructor() {
        super();

        const keywordMapper = this.createKeywordMapper(
            {
                "support.constant": "true|false",
                keyword: "and|else|fun|if|in|let|rec|then|type",
                "keyword.long": "forall|Lam|lam|mu",
                "support.type": "Bool|Bot|Int|Top|Unit",
                "support.function": "foo",
                // "variable.language": "this",
            },
            "identifier",
        );

        this.$rules = {
            start: [
                {
                    token: "comment",
                    regex: /\(\*.*?\*\)\s*?$/,
                },
                {
                    token: "comment",
                    regex: /\(\*.*/,
                    next: "comment",
                },
                {
                    token: "constant.numeric", // integer
                    regex: /(?:0|[1-9]\d*)\b/,
                },
                {
                    token: keywordMapper,
                    regex: /[a-zA-Z_][a-zA-Z0-9_]*\b/,
                },
                {
                    token: "keyword.short",
                    regex: /[∀Λλμ]/,
                },
                {
                    token: "keyword.operator",
                    // NOTE(MH): We need negative lookahead on the minus to
                    // colour -> as punctuation and not as a keyword.
                    regex: /&&|\/(?!\\)|==|[!><]=?|-(?!>)|%|\|\||\+|\*/,
                },
                {
                    token: "keyword.operator.long",
                    regex: /~|\/\\|\\\//,
                },
                {
                    token: "keyword.operator.short",
                    regex: /¬|∩|∪/,
                },
                {
                    token: "punctuation.operator",
                    regex: /->|:|,|=/,
                },
                {
                    token: "paren.lparen",
                    regex: /[()]/,
                },
                {
                    token: "paren.rparen",
                    regex: /[)]/,
                },
                {
                    token: "text",
                    regex: /\s+/,
                },
            ],
            comment: [
                {
                    token: "comment", // closing comment
                    regex: /\*\)/,
                    next: "start",
                },
                {
                    defaultToken: "comment",
                },
            ],
        };
    }
}

export default class LambdaMode extends window.ace.acequire("ace/mode/text").Mode {
    constructor() {
        super();
        this.HighlightRules = LambdaHighlightRules;
        this.$behaviour = this.$defaultBehaviour;
        this.$id = "ace/mode/lambda";
    }

    // We keep this around in case we want something similar later.
    // getCompletions() {
    //     return [{ snippet: "λ", caption: "lambda", meta: "greek" }];
    // }
}
