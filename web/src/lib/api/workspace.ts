import type { InitWorkspace, Workspace } from "$lib/types";
import { api_origin } from "./origin";

export async function publics(): Promise<Workspace[]> {
	const res = await fetch(`${api_origin}/workspace`);
	return await res.json();
}

export async function joined(): Promise<Workspace[]> {
	const res = await fetch(`${api_origin}/workspace/joined`);
	return await res.json();
}

export async function create(data: InitWorkspace): Promise<Workspace> {
	const res = await fetch(`${api_origin}/workspace`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	return await res.json();
}

export async function get(id: number): Promise<Workspace> {
	const res = await fetch(`${api_origin}/workspace/${id}`);
	return await res.json();
}
