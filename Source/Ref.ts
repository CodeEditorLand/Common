/**
 * Ref - reactive mutable state cell.
 *
 * Replaces Effect-TS `Ref` and `SubscriptionRef` with a plain subscribe/notify
 * pattern. Synchronous mutations; async-iterable changes stream.
 */

export interface Ref<T> {
	get(): T;
	set(value: T): void;
	update(fn: (current: T) => T): void;
	subscribe(listener: (value: T) => void): () => void;
	readonly changes: AsyncIterable<T>;
}

export const createRef = <T>(initial: T): Ref<T> => {
	let current = initial;
	const listeners = new Set<(value: T) => void>();

	const notify = (value: T) => {
		for (const fn of listeners) fn(value);
	};

	const ref: Ref<T> = {
		get: () => current,

		set(value) {
			current = value;
			notify(value);
		},

		update(fn) {
			current = fn(current);
			notify(current);
		},

		subscribe(listener) {
			listeners.add(listener);
			return () => listeners.delete(listener);
		},

		get changes() {
			return {
				[Symbol.asyncIterator](): AsyncIterator<T> {
					let resolve: ((r: IteratorResult<T>) => void) | null = null;
					const queue: T[] = [];
					let done = false;

					const unsub = ref.subscribe((v) => {
						if (resolve) {
							const r = resolve;
							resolve = null;
							r({ value: v, done: false });
						} else {
							queue.push(v);
						}
					});

					return {
						next() {
							if (queue.length > 0) {
								return Promise.resolve({
									value: queue.shift() as T,
									done: false,
								});
							}
							if (done) {
								return Promise.resolve({
									value: undefined as unknown as T,
									done: true,
								});
							}
							return new Promise<IteratorResult<T>>((r) => {
								resolve = r;
							});
						},
						return() {
							done = true;
							unsub();
							return Promise.resolve({
								value: undefined as unknown as T,
								done: true,
							});
						},
					};
				},
			};
		},
	};

	return ref;
};

// Alias: createSubscriptionRef is the same as createRef
export const createSubscriptionRef = createRef;
