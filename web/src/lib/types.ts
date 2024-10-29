import type { z } from "zod";
import type {
	ChannelSchema,
	InitMessageSchema,
	InitRoomSchema,
	InitUserSchema,
	InitWorkspaceSchema,
	MessageSchema,
	UserSchema,
	WorkspaceSchema,
} from "./schema";

export type Workspace = z.infer<typeof WorkspaceSchema>;
export type InitWorkspace = z.infer<typeof InitWorkspaceSchema>;

export type User = z.infer<typeof UserSchema>;
export type InitUser = z.infer<typeof InitUserSchema>;

export type InitChannel = z.infer<typeof InitRoomSchema>;
export type Channel = z.infer<typeof ChannelSchema>;
export type InitMessage = z.infer<typeof InitMessageSchema>;
export type Message = z.infer<typeof MessageSchema>;
