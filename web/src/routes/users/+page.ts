import * as api from "$lib/api/user";
import type { PageLoad } from "../workspace/[id=number]/members/$types";

export const load = (async ({ fetch }) => ({
	users: await api.all(fetch),
})) satisfies PageLoad;
