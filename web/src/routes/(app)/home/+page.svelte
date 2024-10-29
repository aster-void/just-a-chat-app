<script lang="ts">
  import { goto } from "$app/navigation";
  import * as ws from "$lib/api/workspace";
  import { onMount } from "svelte";
  import { gotoLoginPage, tokenStore } from "~/lib/api/internal/token-store";
  import type { PageData } from "./$types";
  import Error from "~/components/Error.svelte";

  interface Props {
    data: PageData;
  }

  let { data }: Props = $props();

  onMount(() => {
    if ($tokenStore === undefined) gotoLoginPage();
  });
</script>

<h1>Just a Chat App</h1>

<h2>Joined Workspaces</h2>
<ul>
  {#await data.joined then joined}
    {#each joined as workspace}
      <li>
        {workspace.name}
        <button onclick={() => goto(`/w/${workspace.id}`)}>Go</button>
      </li>
    {/each}
  {:catch err}
    <Error {err} />
  {/await}
</ul>
<button onclick={() => goto("/new")}> Create New </button>
<hr />

<h2>Public Workspaces</h2>
<ul>
  {#await data.publics then publics}
    {#each publics as workspace}
      <li>
        {workspace.name}
        <button
          onclick={() =>
            ws
              .join(fetch, workspace.id)
              .then(() => goto(`/w/${workspace.id}`))
              .catch(console.error)}
        >
          Join
        </button>
      </li>
    {/each}
  {:catch err}
    <Error {err} />
  {/await}
</ul>
