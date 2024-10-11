import { RefObject, useCallback, useEffect, useMemo, useState } from "react";
import AceEditor, { IAnnotation, IMarker } from "react-ace";

import "ace-builds/src-noconflict/mode-rust";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import type * as syntax from "felix-wasm-bridge";
import { vars } from "../theme";
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
            aceRef.current?.editor.gotoLine(1, 0, true);
        },
        [aceRef],
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
            onChange={setProgram}
            onCursorChange={onCursorChange}
            onSelectionChange={onSelectionChange}
            mode="rust"
            theme="github_light_default"
            // theme="github_dark"
            annotations={annotations}
            markers={markers}
            setOptions={{
                // TODO: Make custom language and set up autocompletion.
                // enableBasicAutocompletion: true,
                // enableLiveAutocompletion: true,
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
