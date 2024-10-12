<script lang="ts">
  import Avatar from "~/atoms/svg/Avatar.svelte";
  import Key from "~/atoms/svg/Key.svelte";
  import NavBar from "~/components/NavBar.svelte";
</script>

<NavBar title="Log In"></NavBar>
<main>
  <form class="space-y-2">
    <label class="input input-bordered flex items-center gap-2">
      <Avatar />
      <input type="text" class="grow" placeholder="Username" required />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      <Key />
      <input
        type="password"
        class="grow"
        placeholder="Password"
        required
        minlength="8"
      />
    </label>
    <button type="submit" class="btn btn-primary w-full"> Log In </button>
  </form>
</main>

<style>
  main {
    flex: 1 1 auto;
    align-content: center;
  }
  form {
    width: fit-content;
    height: 200px;
    margin: 0px auto;
  }
</style>
