import { z } from "zod";
import { hashPassword } from "../crypto";
import { Err, Ok, type Result } from "../result";
import { UserSchema } from "../schema";
import type { InitUser, User } from "../types";
import { type Fetcher, POST } from "./internal/fetcher";
import { type Token, tokenStore } from "./internal/token-store";

const LoginResponseSchema = z.object({
	token: z.string(),
	user: UserSchema,
});

export async function signup(
	fetch: Fetcher,
	data: InitUser,
): Promise<Result<User>> {
	try {
		const { rawPassword, ...sendData } = data;
		const res = await POST(fetch, "/signup", 201, LoginResponseSchema, {
			...sendData,
			password: await hashPassword(rawPassword),
		});
		tokenStore.set(res.token as Token);
		return Ok(res.user);
	} catch (err) {
		return Err(err);
	}
}

export { loginFromStale } from "./internal/token-store";

export async function login(
	fetch: Fetcher,
	data: InitUser,
): Promise<Result<User>> {
	try {
		const { rawPassword, ...sendData } = data;
		const res = await POST(fetch, "/login", 200, LoginResponseSchema, {
			...sendData,
			password: await hashPassword(rawPassword),
		});
		tokenStore.set(res.token as Token);
		return Ok(res.user);
	} catch (e) {
		return Err(e);
	}
}
