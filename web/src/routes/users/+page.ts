import * as user from "$lib/api/user";
import type { PageLoad } from "./$types";

export const load = (async ({ fetch }) => ({
	users: await user.all(fetch),
})) satisfies PageLoad;
