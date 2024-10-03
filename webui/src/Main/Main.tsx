import { useEffect, useState } from "react";
import { Tabs } from "@mantine/core";
import { useDebouncedState } from "@mantine/hooks";

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-ocaml";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import Problems from "../Problems/Problems";
import SyntaxTree from "../SyntaxTree/SyntaxTree";

import * as wasm from "felix-wasm-bridge";

import { vars } from "../theme";
import * as classes from "./Main.css";

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
                <AceEditor
                    name="editor"
                    defaultValue={program}
                    focus
                    width="100%"
                    height="100%"
                    onChange={setProgram}
                    mode="ocaml"
                    theme="github_light_default"
                    // theme="github_dark"
                    setOptions={{
                        // TODO: Make custom language and set up autocompletion.
                        // enableBasicAutocompletion: true,
                        // enableLiveAutocompletion: true,
                        fontFamily: vars.fontFamilyMonospace,
                        fontSize: vars.fontSizes.md,
                        newLineMode: "unix",
                        showPrintMargin: false,
                        useSoftTabs: true,
                    }}
                />
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
