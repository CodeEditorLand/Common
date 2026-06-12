/**
 * Phase T1 - EffectSmol runtime contract tests.
 *
 * Gate: all assertions pass before Phase 1 migration begins.
 * Runner: vitest (add `vitest` to devDependencies and run `vitest run`).
 */

import { describe, expect, it } from "vitest";

import {
	createContainer,
	createLayer,
	createMockLayer,
	createToken,
} from "../Source/Container.js";

import { Async } from "../Source/EffectSmol.js";

import { createPubSub } from "../Source/PubSub.js";

import { createRef } from "../Source/Ref.js";

import { Err, isErr, isOk, Ok } from "../Source/Result.js";

// ──────────────────────────────────────────────
// Async (replaces Effect<T, E>)
// ──────────────────────────────────────────────

describe("Async", () => {
	it("succeed / fail round-trip", async () => {
		const r1 = await Promise.resolve(Async.succeed(42));

		expect(r1).toEqual({ ok: true, value: 42 });

		const r2 = await Promise.resolve(Async.fail("NotFound", { id: "x" }));

		expect(r2).toEqual({
			ok: false,
			error: { type: "NotFound", cause: { id: "x" } },
		});
	});

	it("Async.from catches thrown errors", async () => {
		const r = await Async.from(async () => {
			throw new Error("boom");
		});

		expect(r.ok).toBe(false);

		if (!r.ok) expect(r.error.type).toBe("UnhandledError");
	});

	it("Async.try catches sync throw", async () => {
		const r = await Async.try(() => {
			throw new Error("sync");
		});

		expect(r.ok).toBe(false);
	});

	it("Async.from resolves normally", async () => {
		const r = await Async.from(async () => "hello");

		expect(r).toEqual({ ok: true, value: "hello" });
	});

	it("Async.map transforms ok value", async () => {
		const base = Promise.resolve(Async.succeed(10));

		const r = await Async.map(base, (v) => v * 2);

		expect(r).toEqual({ ok: true, value: 20 });
	});

	it("Async.map passes err unchanged", async () => {
		const base = Promise.resolve(Async.fail<"E">("E"));

		const r = await Async.map(base, (v: number) => v * 2);

		expect(r.ok).toBe(false);
	});
});

// ──────────────────────────────────────────────
// Container
// ──────────────────────────────────────────────

describe("Container", () => {
	it("token identity - same name produces same symbol", () => {
		const A = createToken<number>("SameService");

		const B = createToken<number>("SameService");

		expect(A).toBe(B);
	});

	it("token uniqueness - different names are distinct", () => {
		const A = createToken<number>("ServiceA");

		const B = createToken<number>("ServiceB");

		expect(A).not.toBe(B);
	});

	it("register + get synchronous service", () => {
		const Token = createToken<{ value: number }>("SyncSvc");

		const container = createContainer();

		container.register(Token, () => ({ value: 42 }));

		expect(container.get(Token)).toEqual({ value: 42 });
	});

	it("get is memoised (same instance)", () => {
		const Token = createToken<object>("MemoSvc");

		const container = createContainer();

		container.register(Token, () => ({}));

		expect(container.get(Token)).toBe(container.get(Token));
	});

	it("get throws on unknown token", () => {
		const Token = createToken<unknown>("UnknownSvc");

		const container = createContainer();

		expect(() => container.get(Token)).toThrow();
	});

	it("registerAll wires multiple tokens", () => {
		const NumToken = createToken<number>("Num");

		const StrToken = createToken<string>("Str");

		const container = createContainer();

		container.registerAll([
			[NumToken, () => 1],

			[StrToken, () => "hello"],
		]);

		expect(container.get(NumToken)).toBe(1);

		expect(container.get(StrToken)).toBe("hello");
	});

	it("freeze prevents further registration", () => {
		const Token = createToken<number>("FrozenSvc");

		const container = createContainer().freeze();

		expect(() => container.register(Token, () => 1)).toThrow();
	});

	it("createMockLayer overrides via container", () => {
		const Token = createToken<number>("MockSvc");

		const mock = createMockLayer(Token, 99);

		const container = createContainer();

		container.register(mock.token, mock.factory, {
			deps: mock.deps as ServiceToken<unknown>[],
		});

		expect(container.get(Token)).toBe(99);
	});
});

// ──────────────────────────────────────────────
// Ref
// ──────────────────────────────────────────────

describe("createRef", () => {
	it("get returns initial value", () => {
		const ref = createRef(0);

		expect(ref.get()).toBe(0);
	});

	it("set updates value", () => {
		const ref = createRef(0);

		ref.set(5);

		expect(ref.get()).toBe(5);
	});

	it("update applies transform", () => {
		const ref = createRef(2);

		ref.update((v) => v * 3);

		expect(ref.get()).toBe(6);
	});

	it("subscribe notifies on set", () => {
		const ref = createRef(0);

		const values: number[] = [];

		const unsub = ref.subscribe((v) => values.push(v));

		ref.set(1);

		ref.set(2);

		unsub();

		ref.set(3);

		expect(values).toEqual([1, 2]);
	});
});

// ──────────────────────────────────────────────
// PubSub
// ──────────────────────────────────────────────

describe("createPubSub", () => {
	it("fan-out delivers to all subscribers", () => {
		const bus = createPubSub<number>();

		const a: number[] = [];

		const b: number[] = [];

		bus.subscribe((v) => a.push(v));

		bus.subscribe((v) => b.push(v));

		bus.publish(1);

		bus.publish(2);

		expect(a).toEqual([1, 2]);

		expect(b).toEqual([1, 2]);
	});

	it("unsubscribed listener stops receiving", () => {
		const bus = createPubSub<number>();

		const received: number[] = [];

		const unsub = bus.subscribe((v) => received.push(v));

		bus.publish(1);

		unsub();

		bus.publish(2);

		expect(received).toEqual([1]);
	});

	it("subscriberCount tracks live subscriptions", () => {
		const bus = createPubSub<void>();

		expect(bus.subscriberCount).toBe(0);

		const unsub = bus.subscribe(() => {});

		expect(bus.subscriberCount).toBe(1);

		unsub();

		expect(bus.subscriberCount).toBe(0);
	});
});

// ──────────────────────────────────────────────
// Result helpers
// ──────────────────────────────────────────────

describe("Result", () => {
	it("Ok produces AsyncOk shape", () => {
		const r = Ok(42);

		expect(r).toEqual({ ok: true, value: 42 });

		expect(isOk(r)).toBe(true);
	});

	it("Err produces AsyncErr shape", () => {
		const r = Err("NotFound", { id: "x" });

		expect(r.ok).toBe(false);

		expect(isErr(r)).toBe(true);

		if (!r.ok) expect(r.error.type).toBe("NotFound");
	});
});

// TypeScript import only - keep type reference alive
type ServiceToken<T> = ReturnType<typeof createToken<T>>;
