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
@DEFN = DEFN_TYPE | DEFN_TYPE_REC | DEFN_LET | DEFN_LET_REC

DEFN_TYPE = "type" BIND_TYPE
DEFN_TYPE_REC = "type" "rec" BIND_TYPE {"and" BIND_TYPE}

BIND_TYPE = IDENT "=" @TYPE

(* Precendence/associativity of infix operators is given in table below. *)
@TYPE = @LEVEL_TYPE_INFIX
@LEVEL_TYPE_INFIX = @LEVEL_TYPE_PREFIX | TYPE_INFIX
TYPE_INFIX = @LEVEL_TYPE_INFIX OP_TYPE_INFIX @LEVEL_TYPE_INFIX
@LEVEL_TYPE_PREFIX = @LEVEL_TYPE_ATOM | TYPE_PREFIX
TYPE_PREFIX = OP_TYPE_PREFIX @LEVEL_TYPE_PREFIX
@LEVEL_TYPE_ATOM =  TYPE_BUILTIN | TYPE_REF | TYPE_PAREN
TYPE_BUILTIN = "Bot" | "Top" | "Bool" | "Int" | "Unit"
TYPE_REF = IDENT
TYPE_PAREN = "(" @TYPE ")"

OP_TYPE_INFIX = "->" | "\/" | "/\ " | "*"
OP_TYPE_PREFX = "~"

DEFN_LET = "let" BIND_EXPR
DEFN_LET_REC = "let" "rec" BIND_EXPR {"and" BIND_EXPR}

BIND_EXPR = @PAT "=" @EXPR

@PAT = PAT_IDENT | PAT_PAIR
PAT_IDENT = IDENT
PAT_PAIR = "(" @PAT "," @PAT ")"

(* Precendence/associativity of infix operators is given in table below. *)
@EXPR = @LEVEL_EXPR_INFIX
@LEVEL_EXPR_INFIX = @LEVEL_EXPR_PREFIX | EXPR_INFIX
EXPR_INFIX = @LEVEL_EXPR_INFIX OP_EXPR_INFIX @LEVEL_EXPR_INFIX
@LEVEL_EXPR_PREFIX = @LEVEL_POSTFIX | EXPR_PREFIX
EXPR_PREFIX = OP_EXPR_PREFIX @LEVEL_EXPR_PREFIX
@LEVEL_EXPR_APP = @LEVEL_EXPR_ATOM | EXPR_APP
EXPR_APP = @LEVEL_EXPR_APP @LEVEL_EXPR_ATOM
@LEVEL_EXPR_ATOM = EXPR_LIT | EXPR_REF | EXPR_UNIT | EXPR_PAIR | EXPR_FUN | EXPR_LET | EXPR_LET_REC | EXPR_IF | EXPR_PAREN
EXPR_LIT = LIT_NAT | @LIT_BOOL
EXPR_REF = IDENT
EXPR_UNIT = "(" ")"
EXPR_PAIR = "(" @EXPR "," @EXPR ")"
EXPR_FUN = "fun" @PAT "->" @EXPR
EXPR_LET = "let" BIND_EXPR "in" @EXPR
EXPR_LET_REC = "let" "rec" BIND_EXPR {"and" BIND_EXPR} "in" @EXPR
EXPR_IF = "if" @EXPR "then" @EXPR "else" @EXPR
EXPR_PAREN = "(" @EXPR ")"

OP_EXPR_INFIX = "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | "<=" | ">" | ">=" | "&&" | "||"
OP_EXPR_PREFIX = "!"

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"

@LIT_BOOL = "false" | "true"
LIT_NAT = r"0|[1-9][0-9]*"


(* The following rules are only here to record token names: *)

(* Keywords: *)
KW_AND = "and"
KW_ELSE = "else"
KW_FALSE = "false"
KW_FUN = "fun"
KW_IF = "if"
KW_LET = "let"
KW_REC = "rec"
KW_THEN = "then"
KW_TRUE = "true"
KW_TYPE = "type"

(* Builtin types *)
TY_BOOL = "Bool"
TY_BOT = "Bot" (* The empty type. *)
TY_INT = "Int"
TY_TOP = "Top" (* The universal type. *)
TY_UNIT = "Unit"

(* Delimiters *)
RPAREN = ")"
LPAREN = "("

(* Operators & separators *)
AND = "&&"
ARROW = "->"
COMMA = ","
COMPL = "~"
DIV = "/"
EQ = "="
EQ_EQ = "=="
GT = ">"
GT_EQ = ">="
INTER = "/\ "
LT = "<"
LT_EQ = "<="
MINUS = "-"
MOD = "%"
NOT = "!"
NOT_EQ = "!="
OR = "||"
PLUS = "+"
TIMES = "*"
UNION = "\/"
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
            <td>Function type <code>-></code></td>
            <td>right</td>
            <td><code>A -> B -> C</code> means <code>A -> (B -> C)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td>
                <code>A \/ B -> C</code> means <code>(A \/ B) -> C</code><br/>
                <code>A -> B \/ C</code> means <code>A -> (B \/ C)</code>
            </td>
        </tr>
        <tr>
            <td>Union <code>\/</code></td>
            <td>right</td>
            <td><code>A \/ B \/ C</code> means <code>A \/ (B \/ C)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A /\ B \/ C</code> means <code>(A /\ B) \/ C</code></td>
        </tr>
        <tr>
            <td>Intersection <code>/\</code></td>
            <td>right</td>
            <td><code>A /\ B /\ C</code> means <code>A /\ (B /\ C)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>A * B /\ C</code> means <code>(A * B) /\ C</code></td>
        </tr>
        <tr>
            <td>Product <code>*</code></td>
            <td>right</td>
            <td><code>A * B * C</code> means <code>A * (B * C)</code></td>
        </tr>
        <tr>
            <td></td>
            <td></td>
            <td><code>~A * B</code> means <code>(~A) * B</code></td>
        </tr>
        <tr>
            <td>Complement <code>~</code></td>
            <td>right</td>
            <td><code>~~A</code> means <code>~(~A)</code></td>
        </tr>
    </tbody>
</table>

<table>
    <thead>
        <th>Operation</th>
        <th>Associativity</th>
        <th>Example</th>
    </thead>
    <tbody>
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
            <td><code>!A B</code> means <code>!(A B)</code></td>
        </tr>
        <tr>
            <td>Application <code>A B</code></td>
            <td>left</td>
            <td><code>A B C</code> means <code>(A B) C</code></td>
        </tr>
    </tbody>
</table>
