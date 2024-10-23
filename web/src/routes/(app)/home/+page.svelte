<script lang="ts">
import { goto } from "$app/navigation";
import * as ws from "$lib/api/workspace";
import { onMount } from "svelte";
import { gotoLoginPage, tokenStore } from "~/lib/api/internal/token-store";
import type { PageData } from "./$types";

export let data: PageData;

onMount(() => {
	if ($tokenStore === undefined) gotoLoginPage();
});
</script>

<h1>Just a Chat App</h1>

<h2>Joined Workspaces</h2>
<ul>
  {#each data.joined as workspace}
    <li>
      {workspace.name}
      <button on:click={() => goto(`/w/${workspace.id}`)}>Go</button>
    </li>
  {/each}
</ul>
<button on:click={() => goto("/new")}> Create New </button>
<hr />

<h2>Public Workspaces</h2>
<ul>
  {#each data.publics as workspace}
    <li>
      {workspace.name}
      <button
        on:click={() =>
          ws
            .join(fetch, workspace.id)
            .then(() => goto(`/w/${workspace.id}`))
            .catch(console.error)}
      >
        Join
      </button>
    </li>
  {/each}
</ul>
