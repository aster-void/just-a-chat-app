import type { Schema } from "zod";
import { api_origin } from "./origin";
import { assertEq } from "./lib";

export type Fetcher = (url: string, init?: RequestInit) => Promise<Response>;

export async function GET<T>(
	fetch: Fetcher,
	path: string,
	status: number,
	schema: Schema<T>,
): Promise<T> {
	const res = await fetch(`${api_origin}${path}`);
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
}

export async function POST<Send extends {}, Recv extends Send>(
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
		},
		body: JSON.stringify(body),
	});
	assertEq(res.status, status);
	const val = await res.json();
	return schema.parse(val);
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
	});
	assertEq(res.status, 204);
}
