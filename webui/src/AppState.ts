import { createContext, Dispatch, useContext } from "react";
import * as wasm from "felix-wasm-bridge";
import { TreeNodeData } from "@mantine/core";

type GotoCursor = (cursor: wasm.SrcLoc) => void;

export type AppState = {
    program: string;
    syntax: wasm.Element | undefined;
    problems: wasm.Problem[];
    cursor: wasm.SrcLoc;
    cursedSyntax: wasm.Element | null;
    cursedPath: string[];
    hoveredNode: string | null;
    hoveredSyntax: wasm.Element | null;
    treeData: TreeNodeData[];
    elements: Map<string, wasm.Element>;
    gotoCursor: GotoCursor;
};

export type Action =
    | { type: "setProgram"; program: string }
    | { type: "setCursor"; cursor: wasm.SrcLoc }
    | { type: "setHoveredNode"; hoveredNode: string | null }
    | { type: "setGotoCursor"; gotoCursor: GotoCursor };

export const INITIAL_STATE: AppState = {
    program: "",
    syntax: undefined,
    problems: [],
    cursor: { line: -1, column: -1 },
    cursedSyntax: null,
    cursedPath: [],
    hoveredNode: null,
    hoveredSyntax: null,
    treeData: [],
    elements: new Map(),
    gotoCursor: function () {
        console.error("gotoCursor was called before it was set");
    },
};

export function reducer(state: AppState, action: Action): AppState {
    console.debug("reducing", action);
    switch (action.type) {
        case "setProgram":
            return setProgram(state, action.program);
        case "setCursor":
            return setCursor(state, action.cursor);
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

export function init(state: AppState): AppState {
    return setProgram(state, INITIAL_PROGRAM);
}

function setProgram(state: AppState, program: string): AppState {
    if (program === state.program) return state;
    const start = performance.now();
    const { problems, syntax } = wasm.parse(program, {
        include_trivia: false,
    });
    const end = performance.now();
    console.debug(`Parsing took ${Math.ceil(end - start)} ms.`);
    const [treeData, elements] = syntaxToData(syntax);
    return { ...state, program, syntax, problems, treeData, elements };
}

function setCursor(state: AppState, cursor: wasm.SrcLoc): AppState {
    if (cursor.line === state.cursor.line && cursor.column === state.cursor.column) return state;
    let cursedSyntax: wasm.Element | null = null;
    const cursedPath: string[] = [];
    if (state.syntax) {
        cursedSyntax = findCursed(state.syntax, cursor, cursedPath);
    }
    return { ...state, cursor, cursedSyntax, cursedPath };
}

function setHoveredNode(state: AppState, hoveredNode: string | null): AppState {
    if (hoveredNode === state.hoveredNode) return state;
    const hoveredSyntax = hoveredNode !== null ? (state.elements.get(hoveredNode) ?? null) : null;
    return { ...state, hoveredNode, hoveredSyntax };
}

function syntaxToData(root: wasm.Element): [TreeNodeData[], Map<string, wasm.Element>] {
    const elements = new Map();

    function goElement(element: wasm.Element): TreeNodeData {
        elements.set(element.id, element);
        switch (element.tag) {
            case "NODE":
                return goNode(element);
            case "TOKEN":
                return goToken(element);
        }
    }

    function goNode(node: wasm.Node): TreeNodeData {
        return {
            value: node.id,
            label: node.kind,
            children: node.children.map(goElement),
        };
    }

    function goToken(token: wasm.Token): TreeNodeData {
        return {
            value: token.id,
            label: `${token.kind} — ${token.text}`,
        };
    }

    return [[goElement(root)], elements];
}

function before(x: wasm.SrcLoc, y: wasm.SrcLoc): boolean {
    return x.line < y.line || (x.line === y.line && x.column <= y.column);
}

function findCursed(element: wasm.Element, cursor: wasm.SrcLoc, path: string[]): wasm.Element {
    // eslint-disable-next-line no-constant-condition
    while (true) {
        if (element.tag === "TOKEN") {
            return element;
        }
        path.push(element.id);
        // TODO(MH): Use binary search for large counts of children.
        const child = element.children.findLast((x) => before(x.start, cursor));
        if (child === undefined) {
            return element;
        }
        if (child.tag === "NODE" && before(cursor, child.start)) {
            path.push(child.id);
            return child;
        }
        element = child;
    }
}

export const StateContext = createContext<AppState>(INITIAL_STATE);
export const DispatchContext = createContext<Dispatch<Action> | undefined>(undefined);

export function useAppState(): AppState {
    return useContext(StateContext);
}

export function useAppStateDispatch(): Dispatch<Action> {
    const dispatch = useContext(DispatchContext);
    if (!dispatch) {
        console.error("Using useAppStateDispatch outside of AppStateProvider.");
    }
    return dispatch!;
}
