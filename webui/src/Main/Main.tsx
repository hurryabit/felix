import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-ocaml";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import * as classes from "./Main.css";
import { rem } from "@mantine/core";

export default function Main() {
    return <div className={classes.mainColumn}>
        <div className={classes.editorOutputRow}>
            <div className={classes.editorPane}>
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
            <div className={classes.outputPane}></div>
        </div>
        <div className={classes.problemsPane}></div>
    </div>
}
