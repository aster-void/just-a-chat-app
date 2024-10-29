import { z } from "zod";
import { ChannelSchema, MessageSchema } from "../schema";
import type { Channel, InitMessage, Message } from "../types";
import { type Fetcher, GET, POST } from "./internal/fetcher";

const ChannelListSchema = z.array(ChannelSchema);

export function listChannels(
	fetch: Fetcher,
	w: number,
): Promise<Array<Channel>> {
	return GET(fetch, `/${w}/chat`, 200, ChannelListSchema);
}

export function sendMessage(
	fetch: Fetcher,
	w: number,
	chan: number,
	message: InitMessage,
): Promise<Message> {
	return POST(fetch, `/${w}/chat/${chan}/send`, 201, MessageSchema, message);
}
