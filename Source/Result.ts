/**
 * Result - convenience wrappers over AsyncOk / AsyncErr.
 *
 * `Ok(v)` and `Err(type, cause?)` are short-hand constructors so call-sites
 * can return early without spelling out the full discriminated-union shape.
 */

import type { AsyncErr, AsyncOk } from "./EffectSmol.js";

export const Ok = <T>(value: T): AsyncOk<T> =>
	({ ok: true, value }) as const;

export const Err = <E extends string>(
	type: E,
	cause?: unknown,
): AsyncErr<E> => ({ ok: false, error: { type, cause } }) as const;

export const isOk = <T>(
	result: AsyncOk<T> | AsyncErr<string>,
): result is AsyncOk<T> => result.ok === true;

export const isErr = <E extends string>(
	result: AsyncOk<unknown> | AsyncErr<E>,
): result is AsyncErr<E> => result.ok === false;

export const unwrap = <T>(result: AsyncOk<T> | AsyncErr<string>): T => {
	if (!result.ok)
		throw new Error(
			`unwrap() called on Err: ${result.error.type}`,
		);
	return result.value;
};

export const unwrapErr = <E extends string>(
	result: AsyncOk<unknown> | AsyncErr<E>,
): { type: E; cause?: unknown } => {
	if (result.ok)
		throw new Error("unwrapErr() called on Ok");
	return result.error;
};
