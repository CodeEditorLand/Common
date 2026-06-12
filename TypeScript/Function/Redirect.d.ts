import type Interface from "../Interface/Redirect.js";

/**
 * @module Redirect
 *
 */
declare const _default: Interface;

export default _default;

export declare const Response: {

	new (
		body?:
			| import("@cloudflare/workers-types/experimental/index.js").BodyInit
			| null,

		init?: import("@cloudflare/workers-types/experimental/index.js").ResponseInit,
	): import("@cloudflare/workers-types/experimental/index.js").Response;

	prototype: import("@cloudflare/workers-types/experimental/index.js").Response;

	error(): import("@cloudflare/workers-types/experimental/index.js").Response;

	redirect(
		url: string,

		status?: number,
	): import("@cloudflare/workers-types/experimental/index.js").Response;

	json(
		any: any,

		maybeInit?:
			| import("@cloudflare/workers-types/experimental/index.js").ResponseInit
			| import("@cloudflare/workers-types/experimental/index.js").Response,
	): import("@cloudflare/workers-types/experimental/index.js").Response;
};

//# sourceMappingURL=Redirect.d.ts.map
