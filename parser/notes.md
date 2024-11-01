# Technical Notes

This document provides some notes on the technicalities of the parser
implementation.

## Syntax

### Grammar

This is a context free grammar represented in EBNF (i.e. `{X}`/`[X]` make `X`
repeatable/optional). The start symbol is `PROGRAM`. Some productions, like
`LPAREN = "("`, are only included to record that we use `LPAREN` as token name
for the terminal `"("`. In productions of the form
`ID_EXPR = r"_*[a-z][A-Za-z0-9_]*"`, the RHS is a regular expression. Alternatives enclosed in `<...>` produce nodes in the CST, named like the LHS of the production. Alternatives not encloded in `<...>` don't produce nodes in the CST.

```fsharp
PROGRAM = <EXPR>

EXPR = EXPR_ABS | EXPR_APP | EXPR_LET
EXPR_ABS = <"λ" BINDER "." EXPR>
EXPR_APP = <EXPR_APP EXPR_ATOM> | EXPR_ATOM
EXPR_LET = <"let" BINDER "=" EXPR "in" EXPR>
EXPR_ATOM = EXPR_PAREN | EXPR_VAR | EXPR_UNIT
EXPR_PAREN = <"(" EXPR ")">
EXPR_VAR = <ID_EXPR>
EXPR_UNIT = <"unit">

TYPE = TYPE_ARROW
TYPE_ARROW = <TYPE_ATOM "->" TYPE_ARROW> | TYPE_ATOM
TYPE_ATOM = TYPE_PAREN | TYPE_VAR | TYPE_UNIT
TYPE_PAREN = <"(" TYPE ")">
TYPE_VAR = <ID_TYPE>
TYPE_UNIT = <"Unit">

BINDER = <NAME [":" TYPE]>
NAME = <ID_EXPR>

(* The following rules are tokens defined by regular expressions: *)
ID_EXPR = r"_*[a-z][A-Za-z0-9_]*"
ID_TYPE = r"_*[A-Z][A-Za-z0-9_]*"

(* The following rules are only here to record token names: *)

(* Keywords: *)
KW_IN = "in"
KW_LET = "let"
KW_UNIT = "unit"

(* Greek letters: *)
GR_LAMBDA_LOWER = "λ"

(* Builtin types *)
TY_UNIT = "Unit"

(* Delimiters *)
RPAREN = ")"
LPAREN = "("

(* Operators: *)
OP_ARROW = "->"

(* Separators: *)
COLON = ":"
DOT = "."
EQUALS = "="
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
