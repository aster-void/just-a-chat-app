<script lang="ts">
  import * as ws from "$lib/api/workspace";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import type { Workspace } from "$lib/types";

  let workspace: Workspace;
  let err: Error;

  onMount(() => {
    ws.get(parseInt($page.params.id))
      .then((res) => {
        workspace = res;
      })
      .catch((e) => {
        err = e;
      });
  });
</script>

<h2>Workspace</h2>
{#if err}
  <p>Something went wrong: {err.message}</p>
{:else if !workspace}
  Loading...
{:else}
  <p>
    You are in workspace {workspace.name}
  </p>
{/if}
