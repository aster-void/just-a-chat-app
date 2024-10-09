<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

{#each data.users as user}
  <div>
    <span>Id: {user.id}</span>
    <span>Name: {user.name}</span>
  </div>
{/each}
