import type { Response } from "@cloudflare/workers-types/experimental/index.js";

/**
 * @module Redirect
 *
 */
export default interface Interface {
	(URL?: string, Status?: number): Promise<Response>;
}
