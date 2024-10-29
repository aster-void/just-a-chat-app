<script lang="ts">
	import Error from "~/components/Error.svelte";
	import type { PageData } from "./$types";
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
</script>

{#await data.users then users}
	{#each users as user}
		<div>Name: {user.name}</div>
	{/each}
{:catch err}
	<Error {err} />
{/await}
