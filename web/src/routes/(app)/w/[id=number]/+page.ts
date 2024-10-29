import * as chat from "$lib/api/chat";
import * as ws from "$lib/api/workspace";
import type { PageLoad } from "./$types";

export const load = (({ fetch, params }) => {
	const w = Number.parseInt(params.id);
	return {
		workspace: ws.get(fetch, w),
		channels: chat.listChannels(fetch, w),
	};
}) satisfies PageLoad;
