import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-ocaml";
import "ace-builds/src-noconflict/theme-github_dark";
import "ace-builds/src-noconflict/theme-github_light_default";

import { vars } from "../theme";

type Props = {
    program: string;
    setProgram: (program: string) => void;
}

export default function Editor({program, setProgram}: Props) {
    return <AceEditor
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
    />;
}
