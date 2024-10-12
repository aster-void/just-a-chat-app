import type { Schema } from "zod";
import { api_origin } from "./origin";
import { assertEq } from "./lib";
import { token } from "./token-store";

export type Fetcher = (url: string, init?: RequestInit) => Promise<Response>;

async function retry(
	fetch: Fetcher,
	url: string,
	init: RequestInit,
): Promise<Response> {
	if (token) {
		init.headers = {
			"Auth-Token": token,
			...init.headers,
		};
	}
	let res = await fetch(url, init);
	if (res.status === 401 || res.status >= 500) {
		res = await fetch(url, init);
	}
	return res;
}

export async function GET<T>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<T>,
): Promise<T> {
	const res = await retry(fetch, `${api_origin}${path}`, {});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}

export async function PING_STATUS(
	fetch: Fetcher,
	path: string,
): Promise<number> {
	const res = await retry(fetch, `${api_origin}${path}`, {});
	return res.status;
}

export async function POST<Send extends {}, Recv>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<Recv>,
	body: Send,
): Promise<Recv> {
	const res = await retry(fetch, `${api_origin}${path}`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
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
	const res = await retry(fetch, `${api_origin}${path}`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
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
	const res = await retry(fetch, `${api_origin}${path}`, {
		method: "PUT",
		headers: {
			"Content-Type": "application/json",
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
	const res = await retry(fetch, `${api_origin}${path}`, {
		method: "PATCH",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(body),
	});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}

export async function DELETE(fetch: Fetcher, path: string): Promise<void> {
	const res = await retry(fetch, `${api_origin}${path}`, {
		method: "DELETE",
	});
	assertEq(res.status, 204);
}
