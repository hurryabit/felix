import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Ace } from "ace-builds/ace";
import { Range } from "ace-builds/src-noconflict/ace";
import AceEditor, { IAnnotation, IMarker } from "react-ace";

import "ace-builds/src-noconflict/ext-language_tools";
import "ace-builds/src-noconflict/theme-github_light_default";

import type * as syntax from "felix-wasm-bridge";
import { useDebouncedCallback } from "@mantine/hooks";
import { useAppState, useAppStateDispatch } from "../AppState/hooks";
import { vars } from "../theme";
import LambdaMode from "./Lambda";
import * as classes from "./Editor.css";

interface IAcePoint {
    row: number;
    column: number;
}

interface IAceRange {
    start: IAcePoint;
    end: IAcePoint;
}

interface IAceSelection {
    getCursor(): IAcePoint;
    getRange(): IAceRange;
}

interface IAceMouseEvent {
    stopPropagation(): void;
    preventDefault(): void;
    getDocumentPosition(): IAcePoint;
    getButton(): 0 | 1 | 2;
    getAccelKey(): boolean;
}

type SrcLoc = syntax.SrcLoc;

interface SrcSpan {
    start: SrcLoc;
    end: SrcLoc;
}

function pointToLoc(point: IAcePoint): SrcLoc {
    return { line: point.row, column: point.column };
}

function rangeToSpan(range: IAceRange): SrcSpan {
    return { start: pointToLoc(range.start), end: pointToLoc(range.end) };
}

function makeAnnotation(problem: syntax.Problem): IAnnotation {
    return {
        row: problem.start.line,
        column: problem.start.column,
        text: problem.message,
        type: problem.severity.toLowerCase() as "error",
    };
}

function makeMarker(span: SrcSpan, className: string): IMarker {
    return {
        startRow: span.start.line,
        startCol: span.start.column,
        endRow: span.end.line,
        endCol: span.end.column,
        className: className,
        type: "text",
    };
}

function replaceInSession(session: Ace.EditSession, range: Ace.Range, replacement: string) {
    const original = session.getTextRange(range);
    session.replace(range, replacement);
    const undoManager = session.getUndoManager();
    undoManager.add(
        {
            action: "insert",
            start: range.end,
            end: { row: range.end.row, column: range.end.column + replacement.length },
            lines: [replacement],
        },
        false,
        session,
    );
    undoManager.add(
        { action: "remove", start: range.start, end: range.end, lines: [original] },
        true,
        session,
    );
}

const substitutions: Map<string, string> = new Map([
    ["forall", "∀"],
    ["Lam", "Λ"],
    ["lam", "λ"],
    ["mu", "μ"],
    ["/\\", "∧"],
    ["\\/", "∨"],
    ["~", "¬"],
]);

export default function Editor() {
    const self = useRef<AceEditor>(null);
    const { program, problems, inspectedSyntax, hoveredSyntax } = useAppState();
    const dispatch = useAppStateDispatch();
    const setProgram = useDebouncedCallback(function (program: string) {
        dispatch({ type: "setProgram", program });
    }, 50);
    const [selection, setSelection] = useState<SrcSpan | null>(null);
    const annotations = useMemo(
        function () {
            return problems.map(makeAnnotation);
        },
        [problems],
    );
    const markers = useMemo(
        function () {
            const markers: IMarker[] = [];
            if (hoveredSyntax !== null) {
                markers.push(makeMarker(hoveredSyntax, classes.hoveredMarker));
            }
            if (selection !== null) {
                markers.push(makeMarker(selection, classes.selectionMarker));
            }
            if (inspectedSyntax !== null) {
                markers.push(makeMarker(inspectedSyntax, classes.inspectedMarker));
            }
            problems.forEach(function (problem) {
                markers.push(makeMarker(problem, classes.errorMarker));
            });
            return markers;
        },
        [problems, selection, inspectedSyntax, hoveredSyntax],
    );

    useEffect(
        function () {
            function gotoCursor(cursor: SrcLoc) {
                const editor = self.current?.editor;
                if (!editor) {
                    console.error("gotoCursor was called before the editor was loaded");
                    return;
                }
                // NOTE(MH): The +1 is due to an inconsistency in Ace Editor.
                editor.gotoLine(cursor.line + 1, cursor.column, true);
                editor.focus();
            }
            dispatch({ type: "setGotoCursor", gotoCursor });
        },
        [dispatch],
    );

    useEffect(function () {
        const editor = self.current?.editor;
        if (!editor) return;
        const lambdaMode = new LambdaMode();
        editor.session.setMode(lambdaMode as Ace.SyntaxMode);
        editor.session.setUndoSelect(false);
        editor.gotoLine(1, 0, true);
    }, []);

    useEffect(
        function () {
            const editor = self.current?.editor;
            if (!editor) return;
            console.debug("adding onclick handler to editor");
            // NOTE(MH): If dispatch changes, which it shouldn't, we might be
            // leaking event handlers. I don't think this matters in practice.
            editor.addEventListener("click", function (event: IAceMouseEvent) {
                if (event.getButton() !== 0 || !event.getAccelKey()) return;
                event.stopPropagation();
                event.preventDefault();
                dispatch({
                    type: "inspectNodeFromEditor",
                    loc: pointToLoc(event.getDocumentPosition()),
                });
            });
        },
        [dispatch],
    );

    const onChange = useCallback(
        function (value: string, delta: Ace.Delta) {
            setProgram(value);
            if (delta.action !== "insert") {
                return;
            }
            const session = self.current?.editor.session;
            if (!session) return;
            let token;
            let length;

            // Space triggers keyword substitution (e.g. "lam " -> "λ").
            if (delta.lines.length === 1 && delta.lines[0] === " ") {
                token = session.getTokenAt(delta.end.row, delta.end.column - 1);
                if (token === null || token.type !== "keyword.long") {
                    return;
                }
                length = token.value.length + 1;
            } else {
                token = session.getTokenAt(delta.end.row, delta.end.column);
                if (
                    token === null ||
                    token.type !== "keyword.operator.long" ||
                    token.start! + token.value.length !== delta.end.column
                ) {
                    return;
                }
                length = token.value.length;
            }
            const short = substitutions.get(token.value);
            if (short === undefined) {
                console.error(
                    `Found long keyword/operator without substitution: "${token.value}".`,
                );
                return;
            }
            const start = { row: delta.end.row, column: delta.end.column - length };
            replaceInSession(session, Range.fromPoints(start, delta.end), short);
        },
        [setProgram],
    );

    const onSelectionChange = useCallback(function (selection: IAceSelection) {
        setSelection(rangeToSpan(selection.getRange()));
    }, []);

    return (
        <AceEditor
            name="editor"
            ref={self}
            defaultValue={program}
            focus
            width="100%"
            height="100%"
            onChange={onChange}
            onSelectionChange={onSelectionChange}
            mode="text"
            theme="github_light_default"
            annotations={annotations}
            markers={markers}
            setOptions={{
                enableBasicAutocompletion: true,
                enableLiveAutocompletion: false, // We want to trigger autocompletion manually.
                enableMultiselect: false,
                fontFamily: vars.fontFamilyMonospace,
                fontSize: vars.fontSizes.md,
                highlightActiveLine: false,
                highlightSelectedWord: false,
                newLineMode: "unix",
                showPrintMargin: false,
                useSoftTabs: true,
            }}
        />
    );
}
