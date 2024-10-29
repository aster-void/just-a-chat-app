<script lang="ts">
  import type { Channel, Workspace } from "$lib/types";
  import type { Snippet } from "svelte";
    import Error from "~/components/Error.svelte";

  type Props = {
    id: number; // workspace id
    workspace: Promise<Workspace>;
    channels: Promise<Array<Channel>>;
    children: Snippet;
  };
  let { id: workspaceId, workspace, channels, children }: Props = $props();
</script>

{#await workspace}
  loading...
{:then workspace}
  You are in a {workspace.public ? "public" : "private"} workspace {workspace.name}
{:catch err}
  <Error {err} />
{/await}

{@render children()}

{#await channels}
  loading channels...
{:then channels}
  {#each channels as channel (channel.id)}
    {channel.name}
  {/each}
  <a href="/w/{workspaceId}/new-channel">Create New Channel</a>
{/await}
