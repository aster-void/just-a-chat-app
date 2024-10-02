<script lang="ts">
  import { goto } from "$app/navigation";
  import * as ws from "$lib/api/workspace";
</script>

<h1>Just a Chat App</h1>

{#await ws.joined()}
  Loading...
{:then joined}
  {#each joined as workspace}
    - {workspace.name}
  {/each}
{/await}

<button on:click={() => goto("/workspace/create")}> Create New </button>
<hr />
{#await ws.publics()}
  loading...
{:then pub}
  {#each pub as workspace}
    - {workspace.name}
    <button on:click={() => alert(`Joined ${workspace.name}`)}> Join </button>
  {/each}
{/await}
