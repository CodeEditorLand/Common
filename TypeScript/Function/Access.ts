import type Interface from "../Interface/Access.js";
import type Data from "../Interface/Data.js";

/**
 * @module Access
 *
 */
export default (async (...[Key, Identifier, { get }, View]) => {
	try {
		const { Vector, Data } = (await get(Identifier, {
			type: "json",
		})) satisfies unknown as Data;

		return JSON.parse(
			(await import("buffer")).Buffer.from(
				await (
					await import("./Decrypt.js")
				).default(
					await Uint8ArrayFromObject(Data),
					Key ?? "",
					await Uint8ArrayFromObject(Vector),
				),
			).toString(),
		)[View];
	} catch (_Error) {
		console.log(_Error);
	}
}) satisfies Interface as Interface;

export const { default: Uint8ArrayFromObject } = await import(
	"./Uint8ArrayFromObject.js"
);
