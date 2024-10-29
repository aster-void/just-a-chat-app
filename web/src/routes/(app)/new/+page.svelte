<script lang="ts">
	import { goto } from "$app/navigation";
	import * as ws from "$lib/api/workspace";
	import { createForm } from "felte";

	const { form } = createForm({
		async onSubmit(values) {
			const workspace = await ws.create(fetch, values);
			goto(`/w/${workspace.id}`);
		},
	});
</script>

<form use:form>
	<input name="name" required />
	<label>
		Public?
		<input name="public" type="checkbox" />
	</label>
	<button type="submit">Create</button>
</form>
