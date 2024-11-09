import { describe, expect, test } from "vitest";
import { EvalCollector, forTestsOnly, Incremental, Reader, TargetMap, UnsetInput } from "./build";

const { Target } = forTestsOnly;

function OPEN_FILES(_: Reader): string[] {
    throw new UnsetInput(OPEN_FILES);
}

function FILE_TEXT(_: Reader, path: string): string {
    throw new UnsetInput(FILE_TEXT, path);
}

function FILE_NUMS(bs: Reader, path: string): number[] {
    const text = bs.getOrEval(FILE_TEXT, path);
    const res: number[] = [];
    for (let line of text.split("\n")) {
        line = line.trim();
        if (line == "") continue;
        res.push(Number.parseInt(line));
    }
    return res;
}

function FILE_SUM(bs: Reader, path: string): number {
    const list = bs.getOrEval(FILE_NUMS, path);
    return list.reduce((x, y) => x + y, 0);
}

function TOTAL(bs: Reader): number {
    const paths = bs.getOrEval(OPEN_FILES);
    let res = 0;
    for (const path of paths) {
        res += bs.getOrEval(FILE_SUM, path);
    }
    return res;
}

function FILE_BELOW(bs: Reader, path: string, limit: number): boolean {
    const total = bs.getOrEval(FILE_SUM, path);
    return total < limit;
}

describe("Target", () => {
    test("toString", () => {
        expect(Target(OPEN_FILES).toString()).toBe("OPEN_FILES()");
        expect(Target(FILE_TEXT, "foo.txt").toString()).toBe("FILE_TEXT(foo.txt)");
        expect(Target(FILE_BELOW, "foo.txt", 20).toString()).toBe("FILE_BELOW(foo.txt, 20)");
    });
});

describe("TargetMap", () => {
    test("get_set", () => {
        const map = new TargetMap();
        const foo = Target(FILE_BELOW, "tiny", 2);
        const bar = Target(FILE_BELOW, "small", 4);
        const baz = Target(FILE_NUMS, "lower");
        const quux = Target(FILE_NUMS, "upper");

        map.set(foo, "foo");
        expect(map.get(foo)).toBe("foo");
        expect(map.get(bar)).toBeUndefined();
        expect(map.get(baz)).toBeUndefined();
        expect(map.get(quux)).toBeUndefined();

        map.set(bar, "bar");
        expect(map.get(foo)).toBe("foo");
        expect(map.get(bar)).toBe("bar");
        expect(map.get(baz)).toBeUndefined();
        expect(map.get(quux)).toBeUndefined();

        map.set(baz, "baz");
        expect(map.get(foo)).toBe("foo");
        expect(map.get(bar)).toBe("bar");
        expect(map.get(baz)).toBe("baz");
        expect(map.get(quux)).toBeUndefined();

        map.set(foo, "foo1");
        map.set(bar, "bar1");
        map.set(baz, "baz1");
        expect(map.get(foo)).toBe("foo1");
        expect(map.get(bar)).toBe("bar1");
        expect(map.get(baz)).toBe("baz1");
        expect(map.get(quux)).toBeUndefined();
    });
});

describe("Incremental", () => {
    test("OPEN_FILES", () => {
        const bs = new Incremental();
        bs.setInput(OPEN_FILES, ["foo.txt", "bar.txt"]);
        expect(bs.getOrEval(OPEN_FILES)).toEqual(["foo.txt", "bar.txt"]);
    });

    test("FILE_TEXT", () => {
        const bs = new Incremental();
        bs.setInput(FILE_TEXT, "foo.txt", "1000");
        expect(bs.getOrEval(FILE_TEXT, "foo.txt")).toBe("1000");
    });

    test("FILE_NUMS", () => {
        const bs = new Incremental();
        bs.setInput(FILE_TEXT, "foo.txt", "10\n20\n\n 30 ");
        expect(bs.getOrEval(FILE_NUMS, "foo.txt")).toEqual([10, 20, 30]);
    });

    test("FILE_NUMS", () => {
        const bs = new Incremental();
        bs.setInput(FILE_TEXT, "foo.txt", "10\n20\n\n 30 ");
        expect(bs.getOrEval(FILE_SUM, "foo.txt")).toBe(60);
    });

    test("TOTAL", () => {
        const bs = new Incremental();
        bs.setInput(OPEN_FILES, ["foo.txt", "bar.txt"]);
        bs.setInput(FILE_TEXT, "foo.txt", "10\n20\n\n 30 ");
        bs.setInput(FILE_TEXT, "bar.txt", "100");
        expect(bs.getOrEval(TOTAL)).toBe(160);
    });

    test("recalc", () => {
        const collector = EvalCollector();
        const bs = new Incremental({ eventHandler: collector });
        bs.setInput(OPEN_FILES, ["foo.txt", "bar.txt"]);
        bs.setInput(FILE_TEXT, "foo.txt", "10\n20\n\n 30 ");
        bs.setInput(FILE_TEXT, "bar.txt", "100");

        expect(bs.getOrEval(TOTAL)).toBe(160);
        expect(collector.takeTargetsToString()).toEqual([
            "FILE_NUMS(foo.txt)",
            "FILE_SUM(foo.txt)",
            "FILE_NUMS(bar.txt)",
            "FILE_SUM(bar.txt)",
            "TOTAL()",
        ]);

        expect(bs.getOrEval(TOTAL)).toBe(160);
        expect(collector.takeTargetsToString()).toHaveLength(0);

        bs.setInput(FILE_TEXT, "bar.txt", "101");
        expect(bs.getOrEval(TOTAL)).toBe(161);
        expect(collector.takeTargetsToString()).toEqual([
            "FILE_NUMS(bar.txt)",
            "FILE_SUM(bar.txt)",
            "TOTAL()",
        ]);

        bs.setInput(FILE_TEXT, "bar.txt", "\n 101 \n");
        expect(bs.getOrEval(TOTAL)).toBe(161);
        expect(collector.takeTargetsToString()).toEqual(["FILE_NUMS(bar.txt)"]);

        bs.setInput(FILE_TEXT, "bar.txt", "41\n60");
        expect(bs.getOrEval(TOTAL)).toBe(161);
        expect(collector.takeTargetsToString()).toEqual([
            "FILE_NUMS(bar.txt)",
            "FILE_SUM(bar.txt)",
        ]);
    });
});
