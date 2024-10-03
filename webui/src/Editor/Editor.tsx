import { RefObject, useMemo } from "react";

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-ocaml";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import type * as wasm from "felix-wasm-bridge";

import { vars } from "../theme";
import * as classes from "./Editor.css";

type Props = {
    aceRef: RefObject<AceEditor>;
    program: string;
    setProgram: (program: string) => void;
    problems: wasm.Problem[];
    highlightedSpan: { start: wasm.SrcLoc; end: wasm.SrcLoc } | null;
}

export default function Editor({ aceRef, program, setProgram, problems, highlightedSpan }: Props) {
    const annotations = useMemo(function () {
        return problems.map(function (problem) {
            const { line, column } = problem.start;
            return { row: line, column, text: problem.message, type: problem.severity.toLowerCase() };
        });
    }, [problems]);
    const markers = useMemo(function () {
        const markers = problems.map(function (problem) {
            return {
                startRow: problem.start.line,
                startCol: problem.start.column,
                endRow: problem.end.line,
                endCol: problem.end.column,
                className: classes.errorMarker,
                type: "text" as const,
            }
        });
        if (highlightedSpan !== null) {
            markers.push({
                startRow: highlightedSpan.start.line,
                startCol: highlightedSpan.start.column,
                endRow: highlightedSpan.end.line,
                endCol: highlightedSpan.end.column,
                className: classes.highlightMarker,
                type: "text" as const,
            });
        }
        return markers;
    }, [problems, highlightedSpan]);

    return <AceEditor
        name="editor"
        ref={aceRef}
        defaultValue={program}
        focus
        width="100%"
        height="100%"
        onChange={setProgram}
        mode="ocaml"
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
    />;
}
