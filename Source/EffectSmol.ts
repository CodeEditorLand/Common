/**
 * EffectSmol - lightweight Effect-TS replacement for Land.
 *
 * Replaces the `"effect"` npm package with plain TypeScript async/await.
 * No external dependencies. Designed to drop-in replace every Effect-TS
 * call site in Wind, Cocoon, and Sky.
 */

// ──────────────────────────────────────────────
// Async value types (replace Effect<T, E>)
// ──────────────────────────────────────────────

export type AsyncOk<T> = { readonly ok: true; readonly value: T };
export type AsyncErr<E extends string = string> = {
	readonly ok: false;
	readonly error: { readonly type: E; readonly cause?: unknown };
};
export type Async<T, E extends string = string> = Promise<
	AsyncOk<T> | AsyncErr<E>
>;

// ──────────────────────────────────────────────
// Async namespace - mirrors Effect API surface
// ──────────────────────────────────────────────

const succeed = <T>(value: T): AsyncOk<T> => ({ ok: true, value }) as const;

const fail = <E extends string>(type: E, cause?: unknown): AsyncErr<E> =>
	({ ok: false, error: { type, cause } }) as const;

const voidOk: AsyncOk<void> = { ok: true, value: undefined };

const from = async <T>(fn: () => Promise<T>): Async<T> => {
	try {
		return succeed(await fn());
	} catch (cause) {
		return fail("UnhandledError", cause);
	}
};

const tryFn = async <T>(fn: () => T): Async<T> => {
	try {
		return succeed(fn());
	} catch (cause) {
		return fail("UnhandledError", cause);
	}
};

const map = async <T, U, E extends string>(
	a: Async<T, E>,
	fn: (value: T) => U,
): Async<U, E> => {
	const r = await a;

	if (r.ok) return succeed(fn(r.value)) as AsyncOk<U>;

	return r as unknown as AsyncErr<E>;
};

const flatMap = async <T, U, E extends string, F extends string>(
	a: Async<T, E>,
	fn: (value: T) => Async<U, F>,
): Async<U, E | F> => {
	const r = await a;

	if (r.ok) return fn(r.value) as Async<U, E | F>;

	return r as unknown as AsyncErr<E>;
};

const catchError = async <T, E extends string>(
	a: Async<T, E>,
	fn: (error: { type: E; cause?: unknown }) => Async<T>,
): Async<T> => {
	const r = await a;

	if (!r.ok) return fn(r.error as { type: E; cause?: unknown });

	return r as AsyncOk<T>;
};

const withTimeout = async <T, E extends string>(
	a: Async<T, E>,
	ms: number,
): Async<T, E | "Timeout"> => {
	const timeout = new Promise<AsyncErr<"Timeout">>((resolve) =>
		setTimeout(
			() => resolve(fail("Timeout", `timed out after ${ms}ms`)),
			ms,
		),
	);

	return Promise.race([a, timeout]) as Async<T, E | "Timeout">;
};

const sleep = (ms: number): Promise<void> =>
	new Promise((resolve) => setTimeout(resolve, ms));

export const Async = {
	succeed,
	fail,
	void: voidOk,
	from,
	try: tryFn,
	map,
	flatMap,
	catch: catchError,
	timeout: withTimeout,
	sleep,
} as const;
