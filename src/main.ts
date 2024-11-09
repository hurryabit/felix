console.log("initializing...");

const editor = document.querySelector<HTMLTextAreaElement>("#editor")!;
console.assert(editor != null);
const codeSize = document.querySelector<HTMLSpanElement>("#code-size")!;
const codeClear = document.querySelector<HTMLButtonElement>("#code-clear")!;

function handleEditorInput() {
    codeSize.textContent = editor.value.length.toString();
}

function handleCodeClearClick() {
    editor.value = "";
}

editor.addEventListener("input", handleEditorInput);
codeClear.addEventListener("click", handleCodeClearClick);
