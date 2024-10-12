<script>
  import "tailwindcss/tailwind.css";
  import Toast from "../components/toast/Toast.svelte";
</script>

<Toast />
<slot />

<style>
  :global(body) {
    min-height: 100vh;
  }
</style>
