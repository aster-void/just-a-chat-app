import type { PageLoad } from "./$types";

export const load = (async ({ params }) => {
	params;
	return {
		// todo
		users: [],
	};
}) satisfies PageLoad;
