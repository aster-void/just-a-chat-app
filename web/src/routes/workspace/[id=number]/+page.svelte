<script lang="ts">
  import * as ws from "$lib/api/workspace";
  import { page } from "$app/stores";
  const promise = ws.get(parseInt($page.params.id));
  (async () => console.log(await promise))();
</script>

<h2>Workspace</h2>
{#await promise}
  Loading...
{:then workspace}
  <p>
    You are in workspace {workspace.name}
  </p>
{:catch e}
  Something went wrong...
  {e}
{/await}
