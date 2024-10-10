import { RefObject, useCallback, useMemo } from "react";
import AceEditor, { IAnnotation, IMarker } from "react-ace";

import "ace-builds/src-noconflict/mode-rust";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import type * as wasm from "felix-wasm-bridge";
import { vars } from "../theme";
import * as classes from "./Editor.css";

interface Span {
    start: wasm.SrcLoc;
    end: wasm.SrcLoc;
}

function makeAnnotation(problem: wasm.Problem): IAnnotation {
    return {
        row: problem.start.line,
        column: problem.start.column,
        text: problem.message,
        type: problem.severity.toLowerCase() as "error",
    };
}

function makeMarker(span: Span, className: string): IMarker {
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
    setCursor: (loc: wasm.SrcLoc) => void;
    problems: wasm.Problem[];
    highlightedSpan: { start: wasm.SrcLoc; end: wasm.SrcLoc } | null;
};

export default function Editor({
    aceRef,
    program,
    setProgram,
    setCursor,
    problems,
    highlightedSpan,
}: Props) {
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
            if (highlightedSpan !== null) {
                markers.push(makeMarker(highlightedSpan, classes.highlightMarker));
            }
            return markers;
        },
        [problems, highlightedSpan],
    );

    const onCursorChange = useCallback(
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        function (value: any) {
            const cursor = value.getCursor();
            setCursor({ line: cursor.row, column: cursor.column });
        },
        [setCursor],
    );

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
