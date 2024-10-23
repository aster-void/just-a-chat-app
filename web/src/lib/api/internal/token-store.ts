import { writable } from "svelte/store";
import { Err, type Result } from "~/lib/result";

export type Token = string & {
	__INTERNAL_DISALLOW_IMPLICIT_CAST_TO_TOKEN: never;
};

export const leftOff = writable<string | undefined>(undefined);
export const MUST_GOTO_LOGIN_PAGE = writable<boolean>(false);
export function gotoLoginPage() {
	console.log("gotoLoginPage was called");
	MUST_GOTO_LOGIN_PAGE.set(true);
}
export const canAutoLogin = writable<boolean>(false);

export async function loginFromStale(): Promise<Result<void>> {
	return Err(undefined);
}

export const tokenStore = writable<Token | undefined>(undefined);

export let token: Token | undefined;
tokenStore.subscribe((val) => {
	console.log(val);
	token = val;
	canAutoLogin.set(true);
});
