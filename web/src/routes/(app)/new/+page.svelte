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
  <button type="submit">Create</button>
</form>
