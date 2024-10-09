import { GET, type Fetcher } from "./internal/fetcher";
import { UserSchema } from "$lib/schema";
import { z } from "zod";

const UserListSchema = z.array(UserSchema);

export async function myData(fetch: Fetcher) {
	return await GET(fetch, "/users/me", 200, UserSchema);
}

export async function all(fetch: Fetcher) {
	return await GET(fetch, "/users/all", 200, UserListSchema);
}
