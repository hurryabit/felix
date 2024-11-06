import { Element, Node, parse, Problem, SrcLoc, Token } from "felix-wasm-bridge";
import type { TreeNodeData } from "@mantine/core";

type GotoCursor = (cursor: SrcLoc) => void;

export type Inspected = {
    node: string;
    syntax: Element;
    path: string[];
};

export type State = {
    program: string;
    syntax: Element | undefined;
    problems: Problem[];
    inspected: Inspected | null;
    hoveredNode: string | null;
    hoveredSyntax: Element | null;
    treeData: TreeNodeData[];
    elements: Map<string, Element>;
    gotoCursor: GotoCursor;
};

export type Action =
    | { type: "setProgram"; program: string }
    | { type: "inspectNodeFromTree"; node: string | null }
    | { type: "inspectNodeFromEditor"; loc: SrcLoc }
    | { type: "setHoveredNode"; hoveredNode: string | null }
    | { type: "setGotoCursor"; gotoCursor: GotoCursor };

export const INITIAL_STATE: State = {
    program: "",
    syntax: undefined,
    problems: [],
    inspected: null,
    hoveredNode: null,
    hoveredSyntax: null,
    treeData: [],
    elements: new Map(),
    gotoCursor: () => console.error("gotoCursor was called before it was set"),
};

export function reducer(state: State, action: Action): State {
    console.debug("reducing", action);
    switch (action.type) {
        case "setProgram":
            return setProgram(state, action.program);
        case "inspectNodeFromTree":
            return inspectNodeFromTree(state, action.node);
        case "inspectNodeFromEditor":
            return inspectNodeFromEditor(state, action.loc);
        case "setHoveredNode":
            return setHoveredNode(state, action.hoveredNode);
        case "setGotoCursor":
            return { ...state, gotoCursor: action.gotoCursor };
    }
}

const INITIAL_PROGRAM = `# Simple higher order function
let twice =
    λf:Unit -> Unit. λu:Unit. f (f u)
in
twice (λu:Unit. u) unit
`;

export function init(state: State): State {
    return setProgram(state, INITIAL_PROGRAM);
}

function setProgram(state: State, program: string): State {
    if (program === state.program) return state;
    const start = performance.now();
    const { problems, syntax } = parse(program, {
        include_trivia: false,
    });
    const end = performance.now();
    console.debug(`Parsing took ${Math.ceil(end - start)} ms.`);
    const [treeData, elements] = syntaxToData(syntax);
    return { ...state, program, syntax, problems, treeData, elements };
}

function inspectNodeFromTree(state: State, node: string | null): State {
    if (node === state.inspected?.node) return state;
    if (node === null) return { ...state, inspected: null };
    const syntax = state.elements.get(node);
    if (syntax === undefined) return { ...state, inspected: null };
    return { ...state, inspected: { node, syntax, path: [] } };
}

function inspectNodeFromEditor(state: State, loc: SrcLoc): State {
    if (state.syntax === undefined) return state;
    const path: string[] = [];
    const syntax = findCursed(state.syntax, loc, path);
    const node = syntax.id;
    if (node === state.inspected?.node) return state;
    return { ...state, inspected: { node, syntax, path } };
}

function setHoveredNode(state: State, hoveredNode: string | null): State {
    if (hoveredNode === state.hoveredNode) return state;
    const hoveredSyntax = hoveredNode !== null ? (state.elements.get(hoveredNode) ?? null) : null;
    return { ...state, hoveredNode, hoveredSyntax };
}

function syntaxToData(root: Element): [TreeNodeData[], Map<string, Element>] {
    const elements = new Map();

    function goElement(element: Element): TreeNodeData {
        elements.set(element.id, element);
        switch (element.tag) {
            case "NODE":
                return goNode(element);
            case "TOKEN":
                return goToken(element);
        }
    }

    function goNode(node: Node): TreeNodeData {
        return {
            value: node.id,
            label: node.kind,
            children: node.children.map(goElement),
        };
    }

    function goToken(token: Token): TreeNodeData {
        return {
            value: token.id,
            label: `${token.kind} — ${token.text}`,
        };
    }

    return [[goElement(root)], elements];
}

function before(x: SrcLoc, y: SrcLoc): boolean {
    return x.line < y.line || (x.line === y.line && x.column <= y.column);
}

function findCursed(element: Element, loc: SrcLoc, path: string[]): Element {
    // eslint-disable-next-line no-constant-condition
    while (true) {
        if (element.tag === "TOKEN") {
            return element;
        }
        path.push(element.id);
        // TODO(MH): Use binary search for large counts of children.
        const child = element.children.findLast((x) => before(x.start, loc));
        if (child === undefined) {
            return element;
        }
        if (child.tag === "NODE" && before(loc, child.start)) {
            path.push(child.id);
            return child;
        }
        element = child;
    }
}
