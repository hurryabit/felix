import { useEffect, useState } from "react";
import { Tabs } from "@mantine/core";
import { useDebouncedState } from "@mantine/hooks";

import Problems from "../Problems/Problems";
import SyntaxTree from "../SyntaxTree/SyntaxTree";

import * as wasm from "felix-wasm-bridge";

import * as classes from "./Main.css";
import Editor from "../Editor/Editor";

const SAMPLE_PROGRAM = "let f = fun x -> x + x\nin f 2\n";

export default function Main() {
    const [activeTab, setActiveTab] = useState<string | null>("parser")
    const [program, setProgram] = useDebouncedState<string>(SAMPLE_PROGRAM, 200);
    const [problems, setProblems] = useState<wasm.Problem[]>([]);
    const [syntax, setSyntax] = useState<wasm.SyntaxNode>();

    useEffect(function () {
        const { problems, syntax } = wasm.parse(program, { include_trivia: false });
        setProblems(problems);
        setSyntax(syntax);
    }, [program]);

    return <div className={classes.mainColumn}>
        <div className={classes.editorOutputRow}>
            <div className={classes.editorPanel}>
                <Editor program={program} setProgram={setProgram} />
            </div>
            <div className={classes.outputPanel}>
                <Tabs className={classes.outputTabs} value={activeTab} onChange={setActiveTab} inverted>
                    <Tabs.Panel value="parser" flex="1 1 0" mih={0}>
                        <SyntaxTree syntax={syntax} />
                    </Tabs.Panel>
                    <Tabs.Panel value="checker" flex={1}>Panel for the intermediate representation produced by the type checker.</Tabs.Panel>
                    <Tabs.Panel value="interpreter" flex={1}>Panel for the value produced by the interpreter.</Tabs.Panel>
                    <Tabs.List>
                        <Tabs.Tab value="parser">Parser</Tabs.Tab>
                        <Tabs.Tab value="checker">Type checker</Tabs.Tab>
                        <Tabs.Tab value="interpreter">Interpreter</Tabs.Tab>
                    </Tabs.List>
                </Tabs>
            </div>
        </div>
        <div className={classes.problemsPane}>
            <Problems problems={problems} />
        </div>
    </div>
}
