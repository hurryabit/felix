import { useState } from "react";
import { Tabs } from "@mantine/core";
import Editor from "../Editor/Editor";
import Problems from "../Problems/Problems";
import SyntaxTree from "../SyntaxTree/SyntaxTree";
import * as classes from "./Main.css";

export default function Main() {
    const [activeTab, setActiveTab] = useState<string | null>("parser");

    return (
        <div className={classes.mainColumn}>
            <div className={classes.editorOutputRow}>
                <div className={classes.editorPanel}>
                    <Editor />
                </div>
                <div className={classes.outputPanel}>
                    <Tabs
                        className={classes.outputTabs}
                        value={activeTab}
                        onChange={setActiveTab}
                        inverted
                    >
                        <Tabs.Panel value="parser" flex="1 1 0" mih={0}>
                            <SyntaxTree />
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
                <Problems />
            </div>
        </div>
    );
}
