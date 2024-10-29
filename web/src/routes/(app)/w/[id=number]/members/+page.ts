import type { User } from "~/lib/types";
import type { PageLoad } from "./$types";

export const load = (({ params }) => {
	params;
	return {
		// todo
		users: (async () => [] as User[])(),
	};
}) satisfies PageLoad;
