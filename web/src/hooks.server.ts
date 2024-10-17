import type { HandleServerError } from "@sveltejs/kit";

export const handleError = ((input) => {
	if (import.meta.env.DEV) {
		return void console.log({
			status: input.status,
			message: input.message,
		});
	}

	if (input.status === 404) return void console.log("request to unknown path:");

	console.log(input.event, input.message, input.error);
}) satisfies HandleServerError;
