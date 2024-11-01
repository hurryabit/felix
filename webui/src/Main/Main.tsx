import { useCallback, useEffect, useRef, useState } from "react";
import * as wasm from "felix-wasm-bridge";
import type AceEditor from "react-ace";
import { Tabs } from "@mantine/core";
import { useDebouncedState } from "@mantine/hooks";
import Editor from "../Editor/Editor";
import Problems from "../Problems/Problems";
import SyntaxTree from "../SyntaxTree/SyntaxTree";
import * as classes from "./Main.css";

const SAMPLE_PROGRAM = `# Simple higher order function
let twice =
    λf:Unit -> Unit. λu:Unit. f (f u)
in
twice (λu:Unit. u) unit
`;

export default function Main() {
    const [activeTab, setActiveTab] = useState<string | null>("parser");
    const [program, setProgram] = useDebouncedState<string>(SAMPLE_PROGRAM, 20);
    const [cursor, setCursor] = useState<wasm.SrcLoc | undefined>();
    const [problems, setProblems] = useState<wasm.Problem[]>([]);
    const [syntax, setSyntax] = useState<wasm.Element>();
    const editorRef = useRef<AceEditor>(null);
    const [hoveredSyntax, setHoveredSyntax] = useState<wasm.Element | null>(null);
    const [cursedSyntax, setCursedSyntax] = useState<wasm.Element | null>(null);

    useEffect(
        function () {
            const a = performance.now();
            const { problems, syntax } = wasm.parse(program, {
                include_trivia: false,
            });
            const b = performance.now();
            console.debug(`Parsing took ${Math.ceil(b - a)} ms.`);
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
                        hoveredSyntax={hoveredSyntax}
                        cursedSyntax={cursedSyntax}
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
                                setCursedSyntax={setCursedSyntax}
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
