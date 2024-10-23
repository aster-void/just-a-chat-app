<script lang="ts">
import "tailwindcss/tailwind.css";
import Toast from "~/components/toast/Toast.svelte";
/**
 * @typedef {Object} Props
 * @property {import('svelte').Snippet} [children]
 */

/** @type {Props} */
let { children } = $props();
</script>

<Toast />
{@render children?.()}
