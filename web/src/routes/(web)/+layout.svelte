<script lang="ts">
import { page } from "$app/stores";
import NavBar from "~/components/NavBar.svelte";
import { canAutoLogin } from "~/lib/api/internal/token-store";

const titles = new Map<string, string>(
	Object.entries({
		"/login": "Log In",
		"/signup": "Sign Up",
		"": "",
		"/": "",
	}),
);

$: title =
	titles.get($page.url.pathname) ??
	(() => {
		console.error("Unknown path:", $page.url.pathname);
		return "";
	})();
</script>

<NavBar {title}>
  <span>
    <a
      class="btn"
      class:btn-primary={canAutoLogin}
      class:btn-ghost={!canAutoLogin}
      href="/login">Log In</a
    >
    <a
      class="btn"
      class:btn-ghost={canAutoLogin}
      class:btn-primary={!canAutoLogin}
      href="/signup">Sign Up</a
    >
  </span>
</NavBar>
<slot />

<style>
  a {
    margin: 16px;
  }
  :global(body) {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    min-height: 100vh;
  }
</style>
