import type { Schema } from "zod";
import { api_origin } from "./origin";
import { assertEq } from "./lib";
import { token } from "./token-store";

export type Fetcher = (url: string, init?: RequestInit) => Promise<Response>;

export async function GET<T>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<T>,
): Promise<T> {
	const res = await fetch(`${api_origin}${path}`, {
		headers: { "Auth-Token": await token() },
	});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}

export async function POST<Send extends {}, Recv>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<Recv>,
	body: Send,
): Promise<Recv> {
	const res = await fetch(`${api_origin}${path}`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			"Auth-Token": await token(),
		},
		body: JSON.stringify(body),
	});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}
export async function POST_NO_RES<Send extends {}>(
	fetch: Fetcher,
	path: string,
	status: number,
	body: Send,
): Promise<void> {
	const res = await fetch(`${api_origin}${path}`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			"Auth-Token": await token(),
		},
		body: JSON.stringify(body),
	});
	assertEq(res.status, status);
}

export async function PUT<Send extends {}, Recv extends Send>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<Recv>,
	body: Send,
) {
	const res = await fetch(`${api_origin}${path}`, {
		method: "PUT",
		headers: {
			"Content-Type": "application/json",
			"Auth-Token": await token(),
		},
		body: JSON.stringify(body),
	});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}

export async function PATCH<T extends {}>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<T>,
	body: Partial<T>,
) {
	const res = await fetch(`${api_origin}${path}`, {
		method: "PATCH",
		headers: {
			"Content-Type": "application/json",
			"Auth-Token": await token(),
		},
		body: JSON.stringify(body),
	});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}

export async function DELETE(fetch: Fetcher, path: string): Promise<void> {
	const res = await fetch(`${api_origin}${path}`, {
		method: "DELETE",
		headers: { "Auth-Token": await token() },
	});
	assertEq(res.status, 204);
}
