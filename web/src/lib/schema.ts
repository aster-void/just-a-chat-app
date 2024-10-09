import { z } from "zod";

export const WorkspaceSchema = z.object({
	id: z.number(),
	name: z.string().max(255),
});
export const InitWorkspaceSchema = z.object({
	name: z.string(),
});

export const UserSchema = z.object({
	id: z.number(),
	name: z.string(),
});
export const InitUserSchema = z.object({
	name: z.string().max(255),
});
