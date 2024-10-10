import { writable } from "svelte/store";

type Token = string & {
	__INTERNAL_DISALLOW_IMPLICIT_CAST_TO_TOKEN: never;
};

function createToken(): Token {
	return "1" as Token; // todo: ,"exp":"${new Date().getTime() / 1000}"}` as Token; // TODO: use actual token
}

export const tokenStore = writable<Token>(undefined, (set) => {
	setInterval(() => {
		set(createToken());
	});
});
export function refreshToken() {
	tokenStore.set(createToken());
}

let current: Token;
tokenStore.subscribe((val) => {
	current = val;
});

export const token: () => Promise<Token> = () =>
	new Promise((resolve, reject) => {
		let done = false;
		if (current) return resolve(current);
		const unsub = tokenStore.subscribe((val) => {
			if (val) {
				done = true;
				resolve(val);
				unsub();
			}
		});
		setTimeout(() => {
			if (done) return;
			done = true;
			reject("Failed to fetch token: Timeout");
		}, 1000);
	});
