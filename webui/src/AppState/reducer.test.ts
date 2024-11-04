import { expect, test } from "vitest";
import { init, INITIAL_STATE } from "./reducer";

test("init", () => {
    const state = init(INITIAL_STATE);
    expect(state.program).toMatch(/.*let twice =.*in\ntwice.*/s);
    expect(state.syntax?.kind).toBe("PROGRAM");
    expect(state.problems).toHaveLength(0);
    expect(state.treeData).toHaveLength(1);
    expect(state.treeData[0].value).toBe("");
    expect(state.treeData[0].label).toBe("PROGRAM");
    expect(state.treeData[0].children).toHaveLength(1);
    expect(state.elements.get("")).toBe(state.syntax);
});
