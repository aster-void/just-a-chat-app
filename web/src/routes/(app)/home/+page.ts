import * as workspace from "$lib/api/workspace";
import type { PageLoad } from "./$types";

export const load = (({ fetch }) => {
	return {
		joined: workspace.joined(fetch),
		publics: workspace.publics(fetch),
	};
}) satisfies PageLoad;
