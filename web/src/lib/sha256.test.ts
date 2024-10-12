import { sha256 } from "./crypto";
import { test, expect } from "bun:test";

test("sha256 function", async () => {
	expect(await sha256("a test string")).toBe(
		// use `echo -n "a test string" | sha256sum` to get this string
		"b830543dc5d1466110538736d35c37cc61d32076a69de65c42913dfbb1961f46",
	);
});
