<script lang="ts">
  import { goto } from "$app/navigation";
  import * as ws from "$lib/api/workspace";
  import { tokenStore } from "~/lib/api/internal/token-store";
  import type { PageData } from "./$types";
  import { onMount } from "svelte";

  export let data: PageData;

  onMount(() => {
    if ($tokenStore === undefined) goto("/login");
  });
</script>

<h1>Just a Chat App</h1>

<h2>Joined Workspaces</h2>
<ul>
  {#each data.joined as workspace}
    <li>
      {workspace.name}
      <button on:click={() => goto(`/app/w/${workspace.id}`)}>Go</button>
    </li>
  {/each}
</ul>
<button on:click={() => goto("/app/workspace/create")}> Create New </button>
<hr />

<h2>Public Workspaces</h2>
<ul>
  {#each data.publics as workspace}
    <li>
      {workspace.name}
      <button on:click={() => ws.join(fetch, workspace.id)}> Join </button>
    </li>
  {/each}
</ul>
