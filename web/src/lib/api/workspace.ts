import { WorkspaceSchema } from "$lib/schema";
import type { InitWorkspace, Workspace } from "$lib/types";
import { z } from "zod";
import { type Fetcher, GET, POST } from "./internal/fetcher";

const WorkspaceListSchema = z.array(WorkspaceSchema);
export async function publics(fetch: Fetcher): Promise<Workspace[]> {
	return await GET(fetch, "/workspace", 200, WorkspaceListSchema);
}

export async function joined(fetch: Fetcher): Promise<Workspace[]> {
	return await GET(fetch, "/workspace/joined", 200, WorkspaceListSchema);
}

export async function create(
	fetch: Fetcher,
	data: InitWorkspace,
): Promise<Workspace> {
	return await POST(fetch, "/workspace", 201, WorkspaceSchema, data);
}

export async function get(fetch: Fetcher, id: number): Promise<Workspace> {
	return await GET(fetch, `/workspace/${id}`, 200, WorkspaceSchema);
}
