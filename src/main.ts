import { defaultKeymap } from "@codemirror/commands";
import { EditorState } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";

console.log("initializing...");

const ID = {
    editorContainer: "#editor-container",
};

const editorContainer = document.querySelector(ID.editorContainer)!;
console.assert(EditorState !== null, `cannot find ${ID.editorContainer} in DOM.`);
const startState = EditorState.create({
    doc: "Hello World",
    extensions: [keymap.of(defaultKeymap)],
});
const _view = new EditorView({
    state: startState,
    parent: editorContainer,
});
