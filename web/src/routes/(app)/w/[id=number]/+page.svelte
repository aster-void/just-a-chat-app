<script lang="ts">
	import type { PageData } from "./$types";

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
</script>

<h2>Workspace</h2>
<p>
	You are in workspace {data.workspace.name}
</p>
