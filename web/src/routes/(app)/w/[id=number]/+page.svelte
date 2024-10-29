<script lang="ts">
	import { createForm } from "felte";
	import { InitMessageSchema } from "~/lib/schema";
	import { validator } from "@felte/validator-zod";
	import * as chat from "$lib/api/chat";
	import type { PageData } from "./$types";
	import Workspace from "~/pages/Workspace.svelte";
	import { page } from "$app/stores";
	let w = $derived(Number.parseInt($page.params.id));

	interface Props {
		data: PageData;
	}

	let chan = $state(1); // TODO
	const { form } = createForm({
		extend: validator({ schema: InitMessageSchema }),
		onSubmit(values) {
			try {
				const val = InitMessageSchema.parse(values);
				chat.sendMessage(fetch, w, chan, val);
			} catch (err) {
				console.error(err);
			}
		},
	});

	let { data }: Props = $props();
	let { workspace, channels } = data;
</script>

<h2>Workspace</h2>

send to:
<input bind:value={chan} />

<Workspace id={w} {workspace} {channels}>
	<span>Hello, (user name here)!</span>
</Workspace>
<form use:form>
	<input name="content" />
	<button type="submit">Send</button>
</form>
