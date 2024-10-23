<script lang="ts">
import { run } from "svelte/legacy";

import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { onMount } from "svelte";
import { MUST_GOTO_LOGIN_PAGE, leftOff } from "./api/internal/token-store";

let gotoLoginPage = $derived(() => {
	let currentLocation = $page.url.pathname;
	if (currentLocation.length <= 1 || currentLocation === "/login") {
		currentLocation = "/home";
	}
	leftOff.set(currentLocation);
	goto("/login");
});

run(() => {
	if ($MUST_GOTO_LOGIN_PAGE) {
		console.log("goto'ing to /login ...");
		gotoLoginPage?.();
		$MUST_GOTO_LOGIN_PAGE = false;
	}
});

onMount(() => {});
</script>
