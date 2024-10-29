import * as ws from "$lib/api/workspace";
import type { PageLoad } from "./$types";

export const load = (({ fetch, params }) => {
	return {
		workspace: ws.get(fetch, Number.parseInt(params.id)),
	};
}) satisfies PageLoad;
