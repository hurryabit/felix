import deepEqual from "deep-equal";

type IArg = string | number | boolean;
type IArgs = readonly IArg[];
type IValue = unknown;

type Rule<Args extends IArgs, Value extends IValue> = (bs: Reader, ...args: Args) => Value;

export class UnsetInput<Args extends IArgs, Value extends IValue> extends Error {
    readonly rule: Rule<Args, Value>;
    readonly args: Args;

    constructor(rule: Rule<Args, Value>, ...args: Args) {
        super(
            `Input for ${rule.name}(${args.map((arg) => arg.toString()).join(", ")}) should be set.`,
        );
        this.rule = rule;
        this.args = args;
    }
}

export interface Reader {
    getOrEval<Args extends IArgs, Value extends IValue>(
        rule: Rule<Args, Value>,
        ...args: Args
    ): Value;
}

export interface Target {
    readonly rule: Rule<IArgs, IValue>;
    readonly args: IArgs;

    eval(bs: Reader): IValue;
    toString(): string;
}

type ITarget = Target;

function Target<Args extends IArgs, Value extends IValue>(
    rule: Rule<Args, Value>,
    ...args: Args
): Target {
    const irule = rule as Rule<IArgs, IValue>;
    return Object.freeze({
        rule: irule,
        args,

        eval(bs: Reader): IValue {
            return irule(bs, ...args);
        },
        toString(): string {
            return `${rule.name}(${args.map((arg) => arg.toString()).join(", ")})`;
        },
    });
}

interface Traced extends Reader {
    readonly trace: ITarget[];
}

function Traced(wrapped: Reader): Traced {
    const trace: ITarget[] = [];
    return Object.freeze({
        trace,

        getOrEval<Args extends IArgs, Value extends IValue>(
            rule: Rule<Args, Value>,
            ...args: Args
        ): Value {
            const target = Target(rule, ...args);
            trace.push(target);
            return wrapped.getOrEval(rule, ...args);
        },
    });
}

// This class is a workaround for the fact that JavaScript's Map class uses
// identity on keys, which does not work for our use case since values of type
// Target are created ad-hoc in rules. We also cannot simply turn Target
// instances into JSON strings since the rule is a function. Thus, we have two
// layers of maps, one keyed by the rules, which are compared by identity and
// one keyed by the arguments for these rules, converted to JSON strings.
//
// There's a proposal on the way to add immutable tuples and structs to JS:
// https://github.com/tc39/proposal-record-tuple. This will at least solve the
// second part of the problem. If we add a unique symbol to each rule function,
// we can also reduce to a single layer of Maps.
export class TargetMap<T> {
    readonly #data: Map<Rule<IArgs, IValue>, Map<string, T>>;

    constructor() {
        this.#data = new Map();
        Object.freeze(this);
    }

    get(target: ITarget): T | undefined {
        return this.#data.get(target.rule)?.get(JSON.stringify(target.args));
    }

    set(target: ITarget, value: T): TargetMap<T> {
        let inner = this.#data.get(target.rule);
        if (inner == undefined) this.#data.set(target.rule, (inner = new Map()));
        inner.set(JSON.stringify(target.args), value);
        return this;
    }
}

type CacheEntry = {
    value: IValue;
    changedAtRev: number;
    verifiedAtRev: number;
    dependencies: readonly ITarget[];
};

function NewCacheEntry(value: IValue, rev: number, dependencies: readonly ITarget[]): CacheEntry {
    return {
        value,
        changedAtRev: rev,
        verifiedAtRev: rev,
        dependencies,
    };
}

interface EventHandler {
    onSetInput(target: Target): void;
    onEval(target: Target): void;
}

const NullHandler: EventHandler = Object.freeze({
    onSetInput(_: Target): void {},
    onEval(_: Target): void {},
});

type IncrementalOpts = {
    eventHandler?: EventHandler;
};

export class Incremental implements Reader {
    #currentRev: number;
    #cache: TargetMap<CacheEntry>;
    #eventHandler: EventHandler;

    constructor(opts?: IncrementalOpts) {
        this.#currentRev = 0;
        this.#cache = new TargetMap();
        this.#eventHandler = opts?.eventHandler ?? NullHandler;
    }

    setInput<Args extends IArgs, Value extends IValue>(
        rule: Rule<Args, Value>,
        ...argsAndValue: [...Args, Value]
    ): boolean {
        console.assert(
            argsAndValue.length > 0,
            "type systems forces last entry to exist and be of type Value",
        );
        const value = argsAndValue.pop() as Value;
        const args = argsAndValue as unknown as Args;
        const target = Target(rule, ...args);
        const entry = this.#cache.get(target);
        // TODO: We won't need deep equality for all rules. We need to
        // figure out a way to configure this.
        if (entry != undefined && deepEqual(entry.value, value, { strict: true })) {
            return false;
        }
        this.#currentRev += 1;
        this.#cache.set(target, NewCacheEntry(value, this.#currentRev, []));
        this.#eventHandler.onSetInput(target);
        return true;
    }

    getOrEval<Args extends IArgs, Value extends IValue>(
        rule: Rule<Args, Value>,
        ...args: Args
    ): Value {
        return this.#updateEntry(Target(rule, ...args)).value as Value;
    }

    #updateEntry(target: ITarget): CacheEntry {
        const entry = this.#cache.get(target);
        if (entry == undefined) {
            const tr = Traced(this);
            const value = target.eval(tr);
            this.#eventHandler.onEval(target);
            const entry = NewCacheEntry(value, this.#currentRev, tr.trace);
            this.#cache.set(target, entry);
            return entry;
        }

        if (entry.verifiedAtRev == this.#currentRev) {
            return entry;
        }

        if (
            entry.dependencies.every(
                (dep) => this.#updateEntry(dep).changedAtRev <= entry.verifiedAtRev,
            )
        ) {
            entry.verifiedAtRev = this.#currentRev;
            return entry;
        }

        const tr = Traced(this);
        const value = target.eval(tr);
        this.#eventHandler.onEval(target);
        if (!deepEqual(value, entry.value, { strict: true })) {
            entry.value = value;
            entry.changedAtRev = this.#currentRev;
        }
        entry.verifiedAtRev = this.#currentRev;
        entry.dependencies = tr.trace;
        return entry;
    }
}

export interface EvalCollector extends EventHandler {
    takeTargets(): Target[];
    takeTargetsToString(): string[];
}

export function EvalCollector(): EvalCollector {
    const captured = { targets: [] as Target[] };
    function takeTargets(): Target[] {
        const { targets } = captured;
        captured.targets = [];
        return targets;
    }
    return Object.freeze({
        takeTargets,
        takeTargetsToString(): string[] {
            return this.takeTargets().map((t) => t.toString());
        },
        onSetInput(_: Target): void {},
        onEval(target: Target): void {
            captured.targets.push(target);
        },
    });
}

export const forTestsOnly = { Target };
