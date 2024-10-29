<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { MUST_GOTO_LOGIN_PAGE, leftOff } from "./api/internal/token-store";

	let gotoLoginPage = $derived(() => {
		let currentLocation = $page.url.pathname;
		if (currentLocation.length <= 1 || currentLocation === "/login") {
			currentLocation = "/home";
		}
		leftOff.set(currentLocation);
		goto("/login");
	});

	$effect(() => {
		if ($MUST_GOTO_LOGIN_PAGE) {
			console.log("goto'ing to /login ...");
			gotoLoginPage();
			$MUST_GOTO_LOGIN_PAGE = false;
		}
	});
</script>
