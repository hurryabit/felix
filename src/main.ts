import { indentWithTab } from "@codemirror/commands";
import { indentUnit } from "@codemirror/language";
import { EditorState } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";
import { minimalSetup } from "codemirror";
import { lambda, SAMPLE } from "./lambda";

console.log("initializing...");

const ID = {
    editorContainer: "#editor-container",
};
const editorContainer = document.querySelector(ID.editorContainer)!;
console.assert(EditorState !== null, `cannot find ${ID.editorContainer} in DOM.`);

const startState = EditorState.create({
    doc: SAMPLE,
    extensions: [minimalSetup, indentUnit.of("    "), keymap.of([indentWithTab]), lambda()],
});
new EditorView({
    state: startState,
    parent: editorContainer,
});
