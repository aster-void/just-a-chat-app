import { WorkspaceSchema } from "$lib/schema";
import type { InitWorkspace, Workspace } from "$lib/types";
import { z } from "zod";
import { type Fetcher, GET, POST, POST_NO_RES } from "./internal/fetcher";

const WorkspaceListSchema = z.array(WorkspaceSchema);
export function publics(fetch: Fetcher): Promise<Workspace[]> {
	return GET(fetch, "/workspace?joined=false", 200, WorkspaceListSchema);
}

export function joined(fetch: Fetcher): Promise<Workspace[]> {
	return GET(fetch, "/workspace?joined=true", 200, WorkspaceListSchema);
}

export function create(
	fetch: Fetcher,
	data: InitWorkspace,
): Promise<Workspace> {
	return POST(fetch, "/workspace", 201, WorkspaceSchema, data);
}

export function get(fetch: Fetcher, id: number): Promise<Workspace> {
	return GET(fetch, `/${id}/workspace`, 200, WorkspaceSchema);
}

export function join(fetch: Fetcher, id: number): Promise<void> {
	return POST_NO_RES(fetch, `/workspace/join/${id}`, 201, {});
}
