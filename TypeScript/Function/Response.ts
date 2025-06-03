import type Interface from "../Interface/Response.js";

/**
 * @module Response
 *
 */
export default (async (...Option) =>
	new Response(JSON.stringify(Option[0]), {
		status: Option[1] ?? 200,
		headers: {
			"Content-Type": "application/json;charset=utf-8",
		},
	})) satisfies Interface as Interface;

export const { Response } = await import(
	"@cloudflare/workers-types/experimental/index.js"
);
