<script lang="ts">
  import { applyAction } from "$app/forms";
  import { page } from "$app/stores";
  import NavBar from "~/components/NavBar.svelte";

  const titles = new Map<string, string>(Object.entries({
    "/login": 'Log In',
    "/signup": "Sign Up",
    "": "", 
  }));

  $: title = titles.get($page.url.pathname) ?? (() => {
    console.error("Unknown path:", $page.url.pathname);
    return "";
  })();
</script>

<NavBar {title} />
<slot />

<style>
  :global(body) {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    min-height: 100vh;
  }
</style>
