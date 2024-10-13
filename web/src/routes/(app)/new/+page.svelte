<script lang="ts">
  import { createForm } from "felte";
  import * as ws from "$lib/api/workspace";
  import { goto } from "$app/navigation";

  const { form } = createForm({
    async onSubmit(values) {
      const res = await ws.create(fetch, values);
      goto(`/workspace/${res.id}`);
    },
  });
</script>

<form use:form>
  <input name="name" required />
  <button type="submit">Create</button>
</form>
