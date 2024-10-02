import { Tabs } from "@mantine/core";
import AceEditor from "react-ace";
import { useState } from "react";


import "ace-builds/src-noconflict/mode-ocaml";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import * as classes from "./Main.css";
import { rem } from "@mantine/core";

export default function Main() {
    const [activeTab, setActiveTab] = useState<string | null>("parser")

    return <div className={classes.mainColumn}>
        <div className={classes.editorOutputRow}>
            <div className={classes.editorPanel}>
                <AceEditor
                    name="editor"
                    value={"let f = fun x -> x + x\nin f 2\n"}
                    focus
                    width="100%"
                    height="100%"
                    mode="ocaml"
                    theme="github_light_default"
                    // theme="github_dark"
                    setOptions={{
                        // TODO: Make custom language and set up autocompletion.
                        // enableBasicAutocompletion: true,
                        // enableLiveAutocompletion: true,
                        fontSize: rem("1rem"),
                        newLineMode: "unix",
                        showPrintMargin: false,
                        useSoftTabs: true,
                    }}
                />
            </div>
            <div className={classes.outputPanel}>
                <Tabs className={classes.outputTabs} value={activeTab} onChange={setActiveTab} inverted>
                    <Tabs.Panel value="parser" flex={1}>Panel for the concrete syntax tree produced by the parser.</Tabs.Panel>
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
        <div className={classes.problemsPane}></div>
    </div>
}
