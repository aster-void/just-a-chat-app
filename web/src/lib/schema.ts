import { z } from "zod";

export const WorkspaceSchema = z.object({
	id: z.number(),
	name: z.string().max(255),
});
export const InitWorkspaceSchema = z.object({
	name: z.string(),
});

export const UserNameSchema = z.string().min(3).max(255);
export const PasswordSchema = z.string().min(8).max(255);

export const UserSchema = z.object({
	id: z.number(),
	name: UserNameSchema,
});
export const InitUserSchema = z.object({
	name: UserNameSchema,
	password: PasswordSchema,
});
