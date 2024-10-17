# Technical Notes

This document provides some notes on the technicalities of the parser
implementation.

## Syntax

### Grammar

This is a context free grammar represented in EBNF (i.e. `{X}`/`[X]` make `X`
repeatable/optional). The start symbol is `PROGRAM`. Some productions, like
`LPAREN = "("`, are only included to record that we use `LPAREN` as token name
for the terminal `"("`. In productions of the form
`IDENT = r"[A-Za-z_][A-Za-z0-9_]*"`, the RHS is a regular expression.
Non-terminals prefixed with an `@` are "inlined" during parsing and not present
in the resulting CST.

```fsharp
PROGRAM = {DEFN}
@DEFN = DEFN_TYPE | DEFN_FN

DEFN_TYPE = "type" IDENT "=" @TYPE ";"
DEFN_FN = "fn" IDENT SIGNATURE BLOCK

@TYPE = @LEVEL_TYPE_FN
@LEVEL_TYPE_FN = @LEVEL_UNION | TYPE_FN
TYPE_FN = "fn" "(" LIST_TYPES ")" "->" @LEVEL_TYPE_FN
@LEVEL_UNION = @LEVEL_INTERSECTION | TYPE_UNION
TYPE_UNION = @LEVEL_INTERSECTION "|" @LEVEL_UNION
@LEVEL_INTERSECTION = @LEVEL_COMPLEMENT | TYPE_INTERSECTION
TYPE_INTERSECTION = @LEVEL_COMPLEMENT "&" @LEVEL_INTERSECTION
@LEVEL_COMPLEMENT = @LEVEL_BASIC | TYPE_COMPLEMENT
TYPE_COMPLEMENT = "~" @LEVEL_COMPLEMENT
@LEVEL_BASIC = TYPE_BUILTIN | TYPE_REF | TYPE_TUPLE | TYPE_PAREN
TYPE_BUILTIN = "Bool" | "Int" | "Never" | "Any"
TYPE_REF = IDENT
(* A 1-tuple must have a trailing comma! *)
TYPE_TUPLE = "(" [@TYPE "," [@TYPE {"," @TYPE}]] ")"
TYPE_PAREN = "(" @TYPE ")"

LIST_TYPES = [@TYPE {"," @TYPE}]

BLOCK = "{" @BLOCK_INNER "}"
@BLOCK_INNER = {@STMT} [@EXPR]

@STMT = STMT_ASSIGN | STMT_EXPR | STMT_IF | STMT_LET
STMT_ASSIGN = @EXPR "=" @EXPR ";"
STMT_EXPR = @EXPR ";"
STMT_IF = "if" @EXPR BLOCK ["else" (BLOCK | STMT_IF)]
STMT_LET = "let" BINDING ";"
STMT_LET_REC = "let" "rec" BINDING {"and" BINDING} ";"

@EXPR = @LEVEL_TERTIARY
@LEVEL_TERTIARY = @LEVEL_INFIX | EXPR_TERTIARY
EXPR_TERTIARY = @LEVEL_INFIX "?" @LEVEL_TERTIARY ":" @LEVEL_TERTIARY
@LEVEL_INFIX = @LEVEL_PREFIX | EXPR_INFIX
(* Precedence and associativity of infix operators are handled in a later step. *)
EXPR_INFIX = @LEVEL_PREFIX OP_INFIX @LEVEL_INFIX
@LEVEL_PREFIX = @LEVEL_POSTFIX | EXPR_PREFIX
EXPR_PREFIX = OP_PREFIX @LEVEL_PREFIX
@LEVEL_POSTFIX = @LEVEL_ATOM | EXPR_CALL | EXPR_SELECT
EXPR_CALL = @LEVEL_POSTFIX ARGS
EXPR_SELECT = @LEVEL_POSTFIX "." LIT_NAT
@LEVEL_ATOM = EXPR_LIT | EXPR_VAR | EXPR_TUPLE | EXPR_FN | EXPR_PAREN
EXPR_LIT = LIT_NAT | @LIT_BOOL
EXPR_VAR = IDENT
(* A 1-tuple must have a trailing comma! *)
EXPR_TUPLE = "(" [@EXPR "," [@EXPR {"," @EXPR}]] ")"
EXPR_FN = "fn" SIGNATURE BLOCK
EXPR_PAREN = "(" @EXPR ")"

SIGNATURE = "(" [BINDER {"," BINDER}] ")" ["->" @TYPE]
BINDER = ["mut"] IDENT [":" @TYPE]
BINDING = BINDER "=" @EXPR

ARGS = "(" [@EXPR {"," @EXPR}] ")"

OP_INFIX = "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | "<=" | ">" | ">=" | "&&" | "||"
OP_PREFIX = "!"

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"

@LIT_BOOL = "false" | "true"
LIT_NAT = r"0|[1-9][0-9]*"


(* The following rules are only here to record token names: *)

(* Keywords: *)
KW_AND = "and"
KW_ELSE = "else"
KW_FALSE = "false"
KW_FN = "fn"
KW_IF = "if"
KW_LET = "let"
KW_MUT = "mut"
KW_REC = "rec"
KW_TRUE = "true"
KW_TYPE = "type"

(* Builtin types *)
KW_ANY = "Any" (* The top type. *)
KW_BOOL = "Bool"
KW_INT = "Int"
KW_NEVER = "Never" (* The bottom type. *)

(* Delimiters *)
LANGLE = "<"
RANGLE = ">"
LBRACE = "{"
RBRACE = "}"
LBRACKET = "["
RBRACKET = "]"
RPAREN = ")"
LPAREN = "("

(* Operators & separators *)
AMPER = "&"
AMPER_AMPER = "&&"
BANG = "!"
BANG_EQUALS = "!="
BAR = "|"
BAR_BAR = "||"
COLON = ":"
COMMA = ","
DOT = "."
EQUALS = "="
EQUALS_EQUALS = "=="
LANGLE_EQUALS = "<="
MINUS = "-"
MINUS_RANGLE = "->"
QUERY = "?"
RANGLE_EQUALS = ">="
PERCENT = "%"
PLUS = "+"
SEMI = ";"
SLASH = "/"
STAR = "*"
TILDE = "~"
```

### Operator precedence and associativity

The following table lists all operations, from lowest to highest precedence, and
their associativity. (The examples in the rows between two operations involve operations from both rows.)

<table>
    <thead>
        <th>Operation</th>
        <th>Associativity</th>
        <th>Example</th>
    </thead>
    <tbody>
        <tr>
            <td>Tertiary <code>A ? B  : C</code></td>
            <td>right</td>
            <td><code>A ? B : C ? D : E</code> means <code>A ? B : (C ? D : E)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A || B ? C || D ? E || F</code> means <code>(A || B) ? (C || D) : (E || F)</code></td>
        </tr>
        <tr>
            <td>Disjunction <code>||</code></td>
            <td>right</td>
            <td><code>A || B || C</code> means <code>A || (B || C)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A && B || C</code> means <code>(A && B) || C</code></td>
        </tr>
        <tr>
            <td>Conjunction <code>&&</code></td>
            <td>right</td>
            <td><code>A && B && C</code> means <code>A && (B && C)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A == B && C</code> means <code>(A == B) && C</code></td>
        </tr>
        <tr>
            <td>Comparison operators <code>== != < <= > >=</code></td>
            <td>none</td>
            <td><code>A == B == C</code> is invalid</td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A == B + C</code> means <code>A == (B + C)</code></td>
        </tr>
        <tr>
            <td>Additive operators <code>+ -</code></td>
            <td>left</td>
            <td><code>A - B + C</code> means <code>(A - B) + C</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A + B * C</code> means <code>A + (B * C)</code></td>
        </tr>
        <tr>
            <td>Multiplicative operators <code>* / %</code></td>
            <td>left</td>
            <td><code>A / B * C</code> means <code>(A / B) * C</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>!A * B</code> means <code>(!A) * B</code></td>
        </tr>
        <tr>
            <td>Prefix operators <code>!</code></td>
            <td>right</td>
            <td><code>!!A</code> means <code>!(!A)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td>
                <code>!A.0</code> means <code>!(A.0)</code><br />
                <code>!A(B)</code> means <code>!(A(B))</code>
            </td>
        </tr>
        <tr>
            <td>
                Postfix operations <code>A.0 A(B)</code> (select, call)
            </td>
            <td>left</td>
            <td>
                <code>A(B).0</code> means <code>(A(B)).0</code><br />
                <code>A.0(B)</code> means <code>(A.0)(B)</code>
            </td>
        </tr>
    </tbody>
</table>
