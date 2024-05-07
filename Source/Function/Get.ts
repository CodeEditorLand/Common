/**
 * @module Get
 *
 */
export default (async (...[Instance]: Parameters<Interface>) => {
	if (typeof Instance === "string") {
		return Instance;
	}

	const _Map = new Map();

	if (typeof Instance === "object") {
		for (const Key in Instance) {
			if (Object.prototype.hasOwnProperty.call(Instance, Key)) {
				if (
					typeof Instance[Key] === "object" &&
					!Array.isArray(Instance[Key])
				) {
					_Map.set(
						Key,
						await (await import("@Function/Get.js")).default(
							Instance[Key],
						),
					);
				} else {
					_Map.set(Key, Instance[Key]);
				}
			}
		}
	}

	return _Map;
}) satisfies Interface as Interface;

import type Interface from "../Interface/Get.js";
