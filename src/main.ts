import { indentWithTab } from "@codemirror/commands";
import { indentUnit, LanguageSupport, StreamLanguage } from "@codemirror/language";
import { EditorState } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";
import { minimalSetup } from "codemirror";
import { pascal as pascalParser } from "./pascal";

console.log("initializing...");

const ID = {
    editorContainer: "#editor-container",
};
const editorContainer = document.querySelector(ID.editorContainer)!;
console.assert(EditorState !== null, `cannot find ${ID.editorContainer} in DOM.`);

const pascalLanguage = StreamLanguage.define(pascalParser);
const pascal = new LanguageSupport(pascalLanguage);

const startState = EditorState.create({
    doc: "Hello World",
    extensions: [minimalSetup, indentUnit.of("    "), keymap.of([indentWithTab]), pascal],
});
const _view = new EditorView({
    state: startState,
    parent: editorContainer,
});
