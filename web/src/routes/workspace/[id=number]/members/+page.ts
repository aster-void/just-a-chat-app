import { myData } from "$lib/api/user";
import type { PageLoad } from "./$types";

export const load = (async ({ params }) => {
	console.log(params.id);
	return {
		users: [myData],
	};
}) satisfies PageLoad;
