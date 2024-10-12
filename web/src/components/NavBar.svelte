<script lang="ts">
  export let title;
</script>

<header class="w-full navbar bg-base-60 border-10 border-base-500">
  <span class="navbar-start">
    <a href="/" class="btn btn-ghost text-xl">daisyUI</a>
  </span>
  <span class="navbar-center text-3xl"> {title}</span>
  <span class="navbar-end">
    <slot />
  </span>
</header>

<style>
  :global(body) {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
  }
  header {
    position: sticky;
    height: 10vh;
    top: 0;
    backdrop-filter: blur(30px);
  }
</style>
