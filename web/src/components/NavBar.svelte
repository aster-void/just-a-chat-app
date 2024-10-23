<script lang="ts">
interface Props {
	title: string;
	children?: import("svelte").Snippet;
}

let { title, children }: Props = $props();
</script>

<header class="w-full navbar bg-base-60 border-10 border-base-500">
  <span class="navbar-start">
    <a href="/" class="btn btn-ghost text-xl">daisyUI</a>
  </span>
  <span class="navbar-center text-3xl"> {title}</span>
  <span class="navbar-end">
    {@render children?.()}
  </span>
</header>

<style>
  header {
    position: sticky;
    height: 10vh;
    top: 0;
    backdrop-filter: blur(30px);
  }
</style>
