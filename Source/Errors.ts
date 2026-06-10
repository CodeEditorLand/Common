/**
 * Errors - branded error factory helpers.
 *
 * Replaces Effect-TS `Data.TaggedError` with plain discriminated-union errors.
 * Call-sites pattern-match on `error.type` instead of `instanceof`.
 */

export type Tagged<
	E extends string,
	R extends Record<string, unknown> = Record<string, unknown>,
> = {
	readonly type: E;
	readonly cause?: unknown;
} & R;

export const makeError = <
	E extends string,
	R extends Record<string, unknown> = Record<string, unknown>,
>(
	type: E,
	extra?: R & { cause?: unknown },
): Tagged<E, R> =>
	({
		type,
		...extra,
	}) as Tagged<E, R>;

// Common error types used throughout Land
export type UnhandledError = Tagged<"UnhandledError", { message: string }>;
export type TimeoutError = Tagged<"Timeout", { ms: number }>;
export type NotFoundError = Tagged<"NotFound", { id?: string }>;
export type StateLockPoisoned = Tagged<
	"StateLockPoisoned",
	{ context: string }
>;

export const Errors = {
	unhandled: (message: string, cause?: unknown): UnhandledError =>
		makeError("UnhandledError", { message, cause }),

	timeout: (ms: number): TimeoutError => makeError("Timeout", { ms }),

	notFound: (id?: string): NotFoundError => makeError("NotFound", { id }),

	stateLockPoisoned: (context: string): StateLockPoisoned =>
		makeError("StateLockPoisoned", { context }),
} as const;
