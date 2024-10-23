<script>
import Auth from "~/lib/Auth.svelte";
/**
 * @typedef {Object} Props
 * @property {import('svelte').Snippet} [children]
 */

/** @type {Props} */
let { children } = $props();
</script>

<Auth />
{@render children?.()}
