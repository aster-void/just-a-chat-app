import type { User } from "~/lib/types";
import type { PageLoad } from "./$types";

export const load = (async ({ params }) => {
	params;
	return {
		// todo
		users: [] as User[],
	};
}) satisfies PageLoad;
