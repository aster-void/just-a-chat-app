import { UserSchema } from "$lib/schema";
import { type Writable, writable } from "svelte/store";
import { z } from "zod";
import type { User } from "../types";
import { type Fetcher, GET, PING_STATUS } from "./internal/fetcher";

const UserListSchema = z.array(UserSchema);

export async function isAvailable(fetch: Fetcher, username: string) {
	const status = await PING_STATUS(fetch, `/users?name=${username}`);
	switch (status) {
		case 200:
			return false; // someone found: is not available
		case 404:
			return true; // no one found: available
		default:
			throw new Error(
				`api/user.ts::isAvailable - expected 200 or 404, but got ${status}`,
			);
	}
}

export const myDataStore: Writable<User | null> = writable(null);

export async function all(fetch: Fetcher) {
	return await GET(fetch, "/users", 200, UserListSchema);
}
