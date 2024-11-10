import { describe, expect, test } from "vitest";
import { GreenChild, GreenNode, GreenNodeBuilder, GreenToken } from "./green";
import { Kind } from "./kind";

function token(kind: Kind, text: string): GreenToken {
    return new GreenToken(kind, text);
}

function node(kind: Kind, ...children: GreenChild[]) {
    return new GreenNode(kind, children);
}

describe("GreenNodeBuilder", () => {
    test("simple", () => {
        const b = new GreenNodeBuilder();
        b.node("EXPR_VAR", (b) => {
            b.token("ID_EXPR", "xxx");
        });
        expect(b.finish()).toEqual(node("EXPR_VAR", token("ID_EXPR", "xxx")));
    });

    test("straight", () => {
        const b = new GreenNodeBuilder();
        b.node("EXPR_ABS", (b) => {
            b.token("GR_LAM_LOW", "λ");
            b.node("BINDER", (b) => {
                b.token("ID_EXPR", "xxx");
            });
            b.token("DOT", ".");
            b.node("EXPR_VAR", (b) => {
                b.token("ID_EXPR", "xxx");
            });
        });
        expect(b.finish()).toEqual(
            node(
                "EXPR_ABS",
                token("GR_LAM_LOW", "λ"),
                node("BINDER", token("ID_EXPR", "xxx")),
                token("DOT", "."),
                node("EXPR_VAR", token("ID_EXPR", "xxx")),
            ),
        );
    });

    test("checkpoint", () => {
        const b = new GreenNodeBuilder();
        const cp = b.checkpoint();
        b.node("EXPR_VAR", (b) => b.token("ID_EXPR", "xxx"));
        b.node_at(cp, "EXPR_ADD", (b) => {
            b.token("PLUS", "+");
            b.node("EXPR_VAR", (b) => b.token("ID_EXPR", "yyy"));
        });
        expect(b.finish()).toEqual(
            node(
                "EXPR_ADD",
                node("EXPR_VAR", token("ID_EXPR", "xxx")),
                token("PLUS", "+"),
                node("EXPR_VAR", token("ID_EXPR", "yyy")),
            ),
        );
    });

    test("reused checkpoint", () => {
        const b = new GreenNodeBuilder();
        const cp = b.checkpoint();
        b.node("EXPR_VAR", (b) => b.token("ID_EXPR", "xxx"));
        b.node_at(cp, "EXPR_MUL", (b) => {
            b.token("STAR", "*");
            b.node("EXPR_VAR", (b) => b.token("ID_EXPR", "yyy"));
        });
        b.node_at(cp, "EXPR_ADD", (b) => {
            b.token("PLUS", "+");
            b.node("EXPR_VAR", (b) => b.token("ID_EXPR", "zzz"));
        });
        expect(b.finish()).toEqual(
            node(
                "EXPR_ADD",
                node(
                    "EXPR_MUL",
                    node("EXPR_VAR", token("ID_EXPR", "xxx")),
                    token("STAR", "*"),
                    node("EXPR_VAR", token("ID_EXPR", "yyy")),
                ),
                token("PLUS", "+"),
                node("EXPR_VAR", token("ID_EXPR", "zzz")),
            ),
        );
    });

    test("escaped checkpoint", () => {
        const b = new GreenNodeBuilder();
        let cp;
        b.node("NODE", (b) => {
            b.token("TOKEN", "xxx");
            b.token("TOKEN", "yyy");
            cp = b.checkpoint();
        });
        expect(() => b.node_at(cp!, "NODE", () => {})).toThrowError(/invalid.*escape/);
    });

    test("captured checkpoint", () => {
        const b = new GreenNodeBuilder();
        b.node("NODE", (b) => {
            const cp = b.checkpoint();
            b.token("TOKEN", "xxx");
            b.node("NODE", (b) => {
                expect(() => b.node_at(cp!, "NODE", () => {})).toThrowError(/invalid.*captured/);
            });
        });
    });
});
