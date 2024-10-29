import { z } from "zod";

export const WorkspaceSchema = z.object({
	id: z.number(),
	name: z.string().max(255),
	public: z.boolean(),
});
export const InitWorkspaceSchema = z.object({
	name: z.string(),
	public: z.boolean(),
});

export const UserNameSchema = z.string().min(3).max(255);
export const PasswordSchema = z.string().min(8).max(255);

export const UserSchema = z.object({
	id: z.number(),
	name: UserNameSchema,
});
export const InitUserSchema = z.object({
	name: UserNameSchema,
	rawPassword: PasswordSchema,
});

export const InitRoomSchema = z.object({
	name: z.string(),
});
export const ChannelSchema = z.object({
	id: z.number(),
	name: z.string(),
});
export const InitMessageSchema = z.object({
	content: z.string().min(1),
});
export const MessageSchema = z.object({
	id: z.number(),
	content: z.string(),
	posted_at: z.number(), // DateTime<UTC> encoded as UNIX timestamp
	posted_chan: z.number(), // references Channel(id)
	posted_workspace: z.number(), // references Workspace(id)
	posted_by: z.number(), // references User(id)
});
