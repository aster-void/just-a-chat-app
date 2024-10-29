import * as user from "$lib/api/user";
import type { PageLoad } from "./$types";

export const load = (({ fetch }) => ({
	users: user.all(fetch),
})) satisfies PageLoad;
