import type {
	InitWorkspaceSchema,
	WorkspaceSchema,
	InitUserSchema,
	UserSchema,
} from "./schema";
import type { z } from "zod";

export type Workspace = z.infer<typeof WorkspaceSchema>;
export type InitWorkspace = z.infer<typeof InitWorkspaceSchema>;

export type User = z.infer<typeof UserSchema>;
export type InitUser = z.infer<typeof InitUserSchema>;
