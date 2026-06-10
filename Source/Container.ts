/**
 * Container - service token, registration, and DI layer.
 *
 * Replaces Effect-TS `Context.Tag`, `Layer`, and `Effect.gen` service lookups
 * with a simple symbol-keyed memoising container.
 */

import { Async as AsyncNS, type Async } from "./EffectSmol.js";

// ──────────────────────────────────────────────
// Service token (replaces Context.Tag)
// ──────────────────────────────────────────────

export type ServiceToken<T> = symbol & { readonly __kind: "token" };

export const createToken = <T>(name: string): ServiceToken<T> =>
	Symbol.for(`Land.Service.${name}`) as unknown as ServiceToken<T>;

// ──────────────────────────────────────────────
// Service factory type
// ──────────────────────────────────────────────

export type ServiceFactory<T> = (container: ServiceContainer) => T | Promise<T>;

// ──────────────────────────────────────────────
// Layer (replaces Effect Layer)
// ──────────────────────────────────────────────

export interface Layer<T> {
	readonly token: ServiceToken<T>;
	readonly factory: ServiceFactory<T>;
	readonly deps: readonly ServiceToken<unknown>[];
}

export const createLayer = <T>(
	token: ServiceToken<T>,
	factory: ServiceFactory<T>,
	deps: ServiceToken<unknown>[] = [],
): Layer<T> => ({ token, factory, deps });

export const createMockLayer = <T>(
	token: ServiceToken<T>,
	mock: T,
): Layer<T> => ({
	token,
	factory: () => mock,
	deps: [],
});

// ──────────────────────────────────────────────
// Service container (replaces Layer.mergeAll)
// ──────────────────────────────────────────────

export interface ServiceContainer {
	register<T>(
		token: ServiceToken<T>,
		factory: ServiceFactory<T>,
		opts?: { deps?: ServiceToken<unknown>[] },
	): this;
	registerAll(
		entries: Array<
			[
				ServiceToken<unknown>,
				ServiceFactory<unknown>,
				ServiceToken<unknown>[]?,
			]
		>,
	): this;
	get<T>(token: ServiceToken<T>): T;
	freeze(): this;
}

export const createContainer = (): ServiceContainer => {
	const registry = new Map<
		ServiceToken<unknown>,
		{ factory: ServiceFactory<unknown>; deps: ServiceToken<unknown>[] }
	>();
	const cache = new Map<ServiceToken<unknown>, unknown>();
	let frozen = false;

	const container: ServiceContainer = {
		register(token, factory, opts = {}) {
			if (frozen)
				throw new Error(
					`Container is frozen; cannot register ${String(token)}`,
				);
			if (!registry.has(token))
				registry.set(token, { factory, deps: opts.deps ?? [] });
			return this;
		},

		registerAll(entries) {
			for (const [token, factory, deps] of entries)
				this.register(
					token as ServiceToken<unknown>,
					factory as ServiceFactory<unknown>,
					{ deps: deps ?? [] },
				);
			return this;
		},

		get<T>(token: ServiceToken<T>): T {
			if (cache.has(token)) return cache.get(token) as T;

			const entry = registry.get(token);

			if (!entry)
				throw new Error(`Service not registered: ${String(token)}`);

			const value = entry.factory(container);

			if (value instanceof Promise)
				throw new Error(
					`Service factory for ${String(token)} returned a Promise - use async container.build() instead`,
				);

			cache.set(token, value);

			return value as T;
		},

		freeze() {
			frozen = true;
			return this;
		},
	};

	return container;
};

// Mutable variant for tests - wraps a frozen base and allows per-token overrides
export const createMutableContainer = (
	base: ServiceContainer,
): ServiceContainer => {
	const overrides = new Map<ServiceToken<unknown>, unknown>();

	return {
		register(token, factory, opts) {
			overrides.set(token as ServiceToken<unknown>, factory(this));
			return this;
		},
		registerAll(entries) {
			for (const [token, factory] of entries)
				this.register(
					token as ServiceToken<unknown>,
					factory as ServiceFactory<unknown>,
				);
			return this;
		},
		get<T>(token: ServiceToken<T>): T {
			if (overrides.has(token)) return overrides.get(token) as T;
			return base.get(token);
		},
		freeze() {
			return this;
		},
	};
};
