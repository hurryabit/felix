import { useCallback, useEffect, useRef, useState } from "react";
import * as wasm from "felix-wasm-bridge";
import type AceEditor from "react-ace";
import { Tabs } from "@mantine/core";
import { useDebouncedState } from "@mantine/hooks";
import Editor from "../Editor/Editor";
import Problems from "../Problems/Problems";
import SyntaxTree from "../SyntaxTree/SyntaxTree";
import * as classes from "./Main.css";

const SAMPLE_PROGRAM = `// Recursive version of Fibonacci.
fn fib_rec(n) {
    if n <= 1 {
        n
    } else {
        fib_rec(n-1) + fib(n-1)
    }
}

// Loop-via-tail-recusrsion version of Fibonacci.
fn fib_tailrec(n) {
    let mut a = 0;
    let mut b = 1;
    let rec go = |n| {
        if n > 0 {
            let c = b;
            b = a + b;
            a = c;
            go(n-1);
        }
    };
    go(n);
    a
}
`;

export default function Main() {
    const [activeTab, setActiveTab] = useState<string | null>("parser");
    const [program, setProgram] = useDebouncedState<string>(SAMPLE_PROGRAM, 200);
    const [cursor, setCursor] = useDebouncedState<wasm.SrcLoc | undefined>(undefined, 200);
    const [problems, setProblems] = useState<wasm.Problem[]>([]);
    const [syntax, setSyntax] = useState<wasm.Element>();
    const editorRef = useRef<AceEditor>(null);
    const [hoveredSyntax, setHoveredSyntax] = useState<wasm.Element | null>(null);
    const highlightedSpan =
        hoveredSyntax === null ? null : { start: hoveredSyntax.start, end: hoveredSyntax.end };

    useEffect(
        function () {
            const { problems, syntax } = wasm.parse(program, {
                include_trivia: false,
            });
            setProblems(problems);
            setSyntax(syntax);
        },
        [program],
    );

    const onSelectProblem = useCallback(function (problem: wasm.Problem) {
        if (editorRef.current === null) {
            console.warn("Editor not yet loaded.");
            return;
        }
        const editor = editorRef.current?.editor;
        const { line, column } = problem.start;
        // NOTE(MH): The +1 is due to an inconsistency in Ace Editor.
        editor.gotoLine(line + 1, column, true);
        editor.focus();
    }, []);

    return (
        <div className={classes.mainColumn}>
            <div className={classes.editorOutputRow}>
                <div className={classes.editorPanel}>
                    <Editor
                        aceRef={editorRef}
                        program={program}
                        setProgram={setProgram}
                        setCursor={setCursor}
                        problems={problems}
                        highlightedSpan={highlightedSpan}
                    />
                </div>
                <div className={classes.outputPanel}>
                    <Tabs
                        className={classes.outputTabs}
                        value={activeTab}
                        onChange={setActiveTab}
                        inverted
                    >
                        <Tabs.Panel value="parser" flex="1 1 0" mih={0}>
                            <SyntaxTree
                                syntax={syntax}
                                cursor={cursor}
                                setHoveredSyntax={setHoveredSyntax}
                            />
                        </Tabs.Panel>
                        <Tabs.Panel value="checker" flex={1}>
                            Panel for the intermediate representation produced by the type checker.
                        </Tabs.Panel>
                        <Tabs.Panel value="interpreter" flex={1}>
                            Panel for the value produced by the interpreter.
                        </Tabs.Panel>
                        <Tabs.List>
                            <Tabs.Tab value="parser">Parser</Tabs.Tab>
                            <Tabs.Tab value="checker">Type checker</Tabs.Tab>
                            <Tabs.Tab value="interpreter">Interpreter</Tabs.Tab>
                        </Tabs.List>
                    </Tabs>
                </div>
            </div>
            <div className={classes.problemsPane}>
                <Problems problems={problems} onSelect={onSelectProblem} />
            </div>
        </div>
    );
}
