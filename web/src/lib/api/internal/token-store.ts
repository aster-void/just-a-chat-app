import { readable, writable } from "svelte/store";
import { type Result, Err } from "~/lib/result";

export type Token = string & {
	__INTERNAL_DISALLOW_IMPLICIT_CAST_TO_TOKEN: never;
};

// if the user can log in after certain user steps (such as revalidating session or just right in)
export const canLogIn = readable<boolean>(false, (set) => {
	set(true);
});

export const tokenStore = writable<Token | undefined>(undefined);

export let token: Token | undefined;
tokenStore.subscribe((val) => {
	console.log(val);
	token = val;
});

export async function loginFromStale(): Promise<Result<void>> {
	// todo!
	return Err(undefined);
}
