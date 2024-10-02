<script lang="ts">
  import { goto } from "$app/navigation";
  import * as workspace from "$lib/api/workspace";
  import type { Workspace } from "$lib/types";
  import { onMount } from "svelte";

  let joined: Workspace[];
  let joinedErr: Error;

  let publics: Workspace[];
  let publicsErr: Error;

  onMount(() => {
    workspace
      .joined()
      .then((j) => (joined = j))
      .catch((err) => (joinedErr = err));

    workspace
      .publics()
      .then((p) => (publics = p))
      .catch((err) => (publicsErr = err));
  });
</script>

<h1>Just a Chat App</h1>

<h2>Joined Workspaces</h2>
{#if joinedErr}
  Error: {joinedErr}
{:else if !joined}
  Loading...
{:else}
  <ul>
    {#each joined as workspace}
      <li>
        {workspace.name}
        <button on:click={() => goto(`/workspace/${workspace.id}`)}>Go</button>
      </li>
    {/each}
  </ul>
{/if}

<button on:click={() => goto("/workspace/create")}> Create New </button>
<hr />

<h2>Public Workspaces</h2>
{#if publicsErr}
  Error: {publicsErr}
{:else if !publics}
  loading...
{:else}
  <ul>
    {#each publics as workspace}
      <li>
        {workspace.name}
        <button on:click={() => alert(`Joined ${workspace.name}`)}>
          Join
        </button>
      </li>
    {/each}
  </ul>
{/if}
