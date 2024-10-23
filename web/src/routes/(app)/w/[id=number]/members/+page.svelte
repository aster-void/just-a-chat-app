<script lang="ts">
import type { PageData } from "./$types";
export let data: PageData;
</script>

{#each data.users as user}
  <div>Name: {user.name}</div>
{/each}
