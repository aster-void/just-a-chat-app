<script lang="ts">
  import { MUST_GOTO_LOGIN_PAGE, leftOff } from "./api/internal/token-store";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  $: gotoLoginPage = () => {
    let currentLocation = $page.url.pathname;
    if (currentLocation.length <= 1 || currentLocation === "/login") {
      currentLocation = "/home";
    }
    leftOff.set(currentLocation);
    goto("/login");
  };

  $: {
    if ($MUST_GOTO_LOGIN_PAGE) {
      console.log("goto'ing to /login ...");
      gotoLoginPage?.();
      $MUST_GOTO_LOGIN_PAGE = false;
    }
  }

  onMount(() => {});
</script>
