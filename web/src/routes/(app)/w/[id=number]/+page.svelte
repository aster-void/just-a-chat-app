<script lang="ts">
	import type { PageData } from "./$types";

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
</script>

<h2>Workspace</h2>
<p>
	{#await data.workspace}
		loading...
	{:then workspace}
		You are in workspace {workspace.name}
	{/await}
</p>
