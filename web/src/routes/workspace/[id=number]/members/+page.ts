import type { PageLoad } from "./$types";

export const load = (async ({ params }) => {
	console.log(params.id);
	return {
		// todo
		users: [],
	};
}) satisfies PageLoad;
