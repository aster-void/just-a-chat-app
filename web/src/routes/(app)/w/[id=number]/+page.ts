import * as ws from "$lib/api/workspace";
import type { PageLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
	return {
		workspace: await ws.get(fetch, Number.parseInt(params.id)),
	};
}) satisfies PageLoad;
