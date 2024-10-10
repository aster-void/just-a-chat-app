<script lang="ts">
  import { goto } from "$app/navigation";
  import * as ws from "$lib/api/workspace";
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<h1>Just a Chat App</h1>

<h2>Joined Workspaces</h2>
<ul>
  {#each data.joined as workspace}
    <li>
      {workspace.name}
      <button on:click={() => goto(`/workspace/${workspace.id}`)}>Go</button>
    </li>
  {/each}
</ul>
<button on:click={() => goto("/workspace/create")}> Create New </button>
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
