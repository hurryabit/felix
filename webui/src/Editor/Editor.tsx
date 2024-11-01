import { RefObject, useCallback, useEffect, useMemo, useState } from "react";
import { Ace } from "ace-builds/ace";
import { Range } from "ace-builds/src-noconflict/ace";
import AceEditor, { IAnnotation, IMarker } from "react-ace";

import "ace-builds/src-noconflict/ext-language_tools";
import "ace-builds/src-noconflict/theme-github_light_default";

import type * as syntax from "felix-wasm-bridge";
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

type Props = {
    aceRef: RefObject<AceEditor>;
    program: string;
    setProgram: (program: string) => void;
    setCursor: (loc: syntax.SrcLoc) => void;
    problems: syntax.Problem[];
    hoveredSyntax: syntax.Element | null;
    cursedSyntax: syntax.Element | null;
};

export default function Editor({
    aceRef,
    program,
    setProgram,
    setCursor,
    problems,
    hoveredSyntax,
    cursedSyntax,
}: Props) {
    const [selection, setSelection] = useState<SrcSpan | null>(null);
    const annotations = useMemo(
        function () {
            return problems.map(makeAnnotation);
        },
        [problems],
    );
    const markers = useMemo(
        function () {
            const markers = problems.map(function (problem) {
                return makeMarker(problem, classes.errorMarker);
            });
            if (selection !== null) {
                markers.push(makeMarker(selection, classes.selectionMarker));
            }
            if (hoveredSyntax !== null) {
                markers.push(makeMarker(hoveredSyntax, classes.hoveredMarker));
            }
            if (cursedSyntax !== null) {
                markers.push(makeMarker(cursedSyntax, classes.cursedMarker));
            }
            return markers;
        },
        [problems, selection, hoveredSyntax, cursedSyntax],
    );

    useEffect(
        function () {
            const lambdaMode = new LambdaMode();
            const editor = aceRef.current?.editor;
            // @ts-expect-error: LambdaMode is not properly typed anyway.
            editor?.session.setMode(lambdaMode);
            editor?.session.setUndoSelect(false);
            editor?.gotoLine(1, 0, true);
        },
        [aceRef],
    );

    const onChange = useCallback(
        function (value: string, delta: Ace.Delta) {
            setProgram(value);
            if (delta.action !== "insert") {
                return;
            }
            const session = aceRef.current?.editor.session;
            if (session === undefined) {
                console.error("No editor session.");
                return;
            }
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
        [aceRef, setProgram],
    );

    const onCursorChange = useCallback(
        function (selection: IAceSelection) {
            setCursor(pointToLoc(selection.getCursor()));
        },
        [setCursor],
    );

    const onSelectionChange = useCallback(function (selection: IAceSelection) {
        setSelection(rangeToSpan(selection.getRange()));
    }, []);

    return (
        <AceEditor
            name="editor"
            ref={aceRef}
            defaultValue={program}
            focus
            width="100%"
            height="100%"
            onChange={onChange}
            onCursorChange={onCursorChange}
            onSelectionChange={onSelectionChange}
            mode="text"
            theme="github_light_default"
            annotations={annotations}
            markers={markers}
            setOptions={{
                enableBasicAutocompletion: true,
                enableLiveAutocompletion: false, // We want to trigger autocompletion manually.
                fontFamily: vars.fontFamilyMonospace,
                fontSize: vars.fontSizes.md,
                highlightActiveLine: false,
                newLineMode: "unix",
                showPrintMargin: false,
                useSoftTabs: true,
            }}
        />
    );
}
