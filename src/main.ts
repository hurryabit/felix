import { indentWithTab } from "@codemirror/commands";
import { indentUnit, LanguageSupport, StreamLanguage } from "@codemirror/language";
import { EditorState } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";
import { minimalSetup } from "codemirror";
import { pascal as pascalParser } from "./pascal";

console.log("initializing...");

const CODE = `while a <> b do  WriteLn('Waiting');

if a > b then WriteLn('Condition met')   {no semicolon allowed before else}
    else WriteLn('Condition not met');

for i := 1 to 10 do  {no semicolon here as it would detach the next statement}
    WriteLn('Iteration: ', i);

repeat
    a := a + 1
until a = 10;

case i of
    0 : Write('zero');
    1 : Write('one');
    2 : Write('two');
    3,4,5,6,7,8,9,10: Write('?')
end;

while a <> b do  WriteLn('Waiting');

if a > b then WriteLn('Condition met')   {no semicolon allowed before else}
    else WriteLn('Condition not met');

for i := 1 to 10 do  {no semicolon here as it would detach the next statement}
    WriteLn('Iteration: ', i);

repeat
    a := a + 1
until a = 10;

case i of
    0 : Write('zero');
    1 : Write('one');
    2 : Write('two');
    3,4,5,6,7,8,9,10: Write('?')
end;`

const ID = {
    editorContainer: "#editor-container",
};
const editorContainer = document.querySelector(ID.editorContainer)!;
console.assert(EditorState !== null, `cannot find ${ID.editorContainer} in DOM.`);

const pascalLanguage = StreamLanguage.define(pascalParser);
const pascal = new LanguageSupport(pascalLanguage);

const startState = EditorState.create({
    doc: CODE,
    extensions: [minimalSetup, indentUnit.of("    "), keymap.of([indentWithTab]), pascal],
});
const _view = new EditorView({
    state: startState,
    parent: editorContainer,
});
