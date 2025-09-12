import type Interface from "../Interface/Redirect.js";

/**
 * @module Redirect
 *
 */
export default (async (...Option) =>
	Response.redirect(
		Option[0] ?? "",
		Option[1] ?? 302,
	)) satisfies Interface as Interface;

export const { Response } = await import(
	"@cloudflare/workers-types/experimental/index.js"
);
