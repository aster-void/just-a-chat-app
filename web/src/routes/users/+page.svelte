<script lang="ts">
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

let { data }: Props = $props();
</script>

{#each data.users as user}
  <div>
    <span>Id: {user.id}</span>
    <span>Name: {user.name}</span>
  </div>
{/each}
