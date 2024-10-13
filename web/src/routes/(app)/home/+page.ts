import type { PageLoad } from "./$types";
import * as workspace from "$lib/api/workspace";

export const load = (async ({ fetch }) => {
	return {
		joined: await workspace.joined(fetch),
		publics: await workspace.publics(fetch),
	};
}) satisfies PageLoad;
