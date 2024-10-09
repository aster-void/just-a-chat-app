const shouldPanic = import.meta.env.DEV;

export function assertEq<T>(got: T, expect: T, ctx?: string) {
	if (got === expect) return;
	const err = new Error(
		`assertEq: expected ${JSON.stringify(expect)}, but got ${JSON.stringify(got)} in ${ctx ?? "(no context provided)"}`,
	);

	if (shouldPanic) throw err;
	console.error(err);
}
