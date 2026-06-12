/**
 * PubSub - fan-out event bus.
 *
 * Replaces Effect-TS `PubSub` with a simple set of subscriber callbacks.
 * Synchronous fan-out; no back-pressure.
 */

export interface PubSub<T> {
	publish(value: T): void;

	subscribe(listener: (value: T) => void): () => void;

	readonly subscriberCount: number;
}

export const createPubSub = <T>(): PubSub<T> => {
	const listeners = new Set<(value: T) => void>();

	return {
		publish(value) {
			for (const fn of listeners) fn(value);
		},

		subscribe(listener) {
			listeners.add(listener);

			return () => listeners.delete(listener);
		},

		get subscriberCount() {
			return listeners.size;
		},
	};
};
