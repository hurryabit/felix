import { Kind } from "./kind";

export class GreenNode {
    readonly kind: Kind;
    readonly length: number; // Length of the text below this node.
    readonly children: readonly GreenChild[];

    constructor(kind: Kind, children: readonly GreenChild[]) {
        this.kind = kind;
        this.length = children.reduce((n, c) => n + c.length, 0);
        this.children = children;
    }

    debug(): string {
        const buffer = [""] as string[];
        this.debugInto(buffer, "");
        buffer.push("");
        return buffer.join("\n");
    }

    debugInto(buffer: string[], indent: string): void {
        buffer.push(`${indent}${this.kind}#${this.length}`);
        indent = indent + "  ";
        this.children.forEach((c) => c.debugInto(buffer, indent));
    }
}

export class GreenToken {
    readonly kind: Kind;
    readonly text: string;

    constructor(kind: Kind, text: string) {
        this.kind = kind;
        this.text = text;
    }

    get length(): number {
        return this.text.length;
    }

    debugInto(buffer: string[], indent: string): void {
        let text = this.text;
        if (/\s/.test(text)) text = encodeURIComponent(text);
        buffer.push(`${indent}${this.kind}#${this.length} :: ${this.text}`);
    }
}

export type GreenChild = GreenNode | GreenToken;

export type Checkpoint = number & { readonly tag: unique symbol };

export class GreenNodeBuilder {
    readonly #interner: Interner;
    readonly #parents: [Kind, number][];
    readonly #children: WithHash<GreenChild>[];

    constructor() {
        this.#interner = new Interner();
        this.#parents = [];
        this.#children = [];
    }

    token(kind: Kind, text: string): void {
        this.#children.push(this.#interner.token(kind, text));
    }

    node(kind: Kind, f: (b: GreenNodeBuilder) => void): void {
        this.#start_node(kind);
        f(this);
        this.#finish_node();
    }

    checkpoint(): Checkpoint {
        return this.#children.length as Checkpoint;
    }

    node_at(checkpoint: Checkpoint, kind: Kind, f: (b: GreenNodeBuilder) => void): void {
        this.#start_node_at(checkpoint, kind);
        f(this);
        this.#finish_node();
    }

    #start_node(kind: Kind) {
        this.#parents.push([kind, this.#children.length]);
    }

    #finish_node() {
        const [kind, start_index] = this.#parents.pop()!;
        const hash_node = this.#interner.node(kind, this.#children, start_index);
        this.#children.push(hash_node);
    }

    #start_node_at(checkpoint: Checkpoint, kind: Kind) {
        if (checkpoint > this.#children.length)
            throw new Error("invalid checkpoint: did it escape its scope?");
        const num_parents = this.#parents.length;
        if (num_parents > 0 && checkpoint < this.#parents[num_parents - 1][1])
            throw new Error("invalid checkpoint: was is captured in a nested scope?");
        this.#parents.push([kind, checkpoint]);
    }

    finish(): GreenNode {
        if (this.#children.length != 1) throw new Error("wrong number of children in finish");
        const { it } = this.#children.pop()!;
        if (!(it instanceof GreenNode)) throw new Error("bad node in finish");
        return it;
    }
}

type WithHash<T> = { readonly hash: number; readonly it: T };

class Interner {
    node(kind: Kind, elements: WithHash<GreenChild>[], start_index: number): WithHash<GreenNode> {
        // TODO(MH): Do the interning.
        console.assert(start_index <= elements.length);
        const children = new Array<GreenChild>(elements.length - start_index);
        let i = children.length;
        while (i != 0) {
            i--;
            children[i] = elements.pop()!.it;
        }
        return {
            hash: 0,
            it: new GreenNode(kind, children),
        };
    }

    token(kind: Kind, text: string): WithHash<GreenToken> {
        // TODO(MH): Do the interning.
        return { hash: 0, it: new GreenToken(kind, text) };
    }
}
