import { LanguageSupport, StreamLanguage, StreamParser, StringStream } from "@codemirror/language";
import { Tag, tags } from "@lezer/highlight";

export const SAMPLE: string = `# Simple higher order function
let twice =
    λf:Unit -> Unit. λu:Unit. f (f u)
in
twice (λu:Unit. u) unit
`;

type TokenKindInfo = { regexp: RegExp; tag: Tag | null };

// NOTE(MH): We don't add a type annotation such that we can use the inferred
// key type further down. We then narrow the type explicitly using this inferred
// type.
const TOKEN_KINDS0 = {
    WHITESPACE: { regexp: /\s+/, tag: null },

    KW_IN: { regexp: /in\b/, tag: tags.definitionKeyword },
    KW_LET: { regexp: /let\b/, tag: tags.definitionKeyword },

    GR_LAM_LOW: { regexp: /λ/, tag: tags.keyword },

    TY_UNIT: { regexp: /Unit\b/, tag: tags.typeName },

    LIT_UNIT: { regexp: /unit\b/, tag: tags.atom },
    LIT_NAT: { regexp: /(?:0|[1-9][0-9]*)\b/, tag: tags.integer },

    ID_EXPR: { regexp: /_*[a-z][a-zA-Z0-9]*/, tag: tags.variableName },
    ID_TYPE: { regexp: /_*[A-Z][a-zA-Z0-9]*/, tag: tags.typeName },

    OP_ARROW: { regexp: /->/, tag: tags.typeOperator },
    OP_INTER: { regexp: /\/\\/, tag: tags.typeOperator },
    OP_UNION: { regexp: /\\\//, tag: tags.typeOperator },

    COLON: { regexp: /:/, tag: tags.punctuation },
    DOT: { regexp: /\./, tag: tags.punctuation },
    EQUALS: { regexp: /=/, tag: tags.punctuation },

    LPAREN: { regexp: /\(/, tag: tags.paren },
    RPAREN: { regexp: /\)/, tag: tags.paren },

    COMMENT: { regexp: /#.*$/m, tag: tags.comment },
    UNKNOWN: { regexp: /./, tag: tags.invalid },
};

type TokenKind = keyof typeof TOKEN_KINDS0;

const TOKEN_KINDS: Record<TokenKind, TokenKindInfo> = TOKEN_KINDS0;

type State = null;

const lambdaParser: StreamParser<State> = {
    startState: () => null,
    token: (stream: StringStream, _: State): string | null => {
        // FIXME(MH): Linear search is ridiculuous but unfortunately JS' RegExp
        // class does not allow for the same simple approach as Python:
        // https://docs.python.org/3/library/re.html#writing-a-tokenizer. Let's
        // improve this once we have more pieces in place.
        for (const kind in TOKEN_KINDS) {
            const { regexp, tag } = TOKEN_KINDS[kind as TokenKind];
            // TODO(MH): Codemirror's typing of StringStream.match could benefit
            // from some improvement. Let's contribute this.
            const match = stream.match(regexp) as RegExpMatchArray | null;
            if (match != null) return tag?.toString() ?? null;
        }
        throw new Error("Lambda tokenizer fell through the floor! 😢");
    },
};
const lambdaLanguage = StreamLanguage.define(lambdaParser);

export function lambda(): LanguageSupport {
    return new LanguageSupport(lambdaLanguage);
}

export const forTestsOnly = { lambdaParser };
