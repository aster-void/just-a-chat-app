<script lang="ts">
  import { goto } from "$app/navigation";
  import Avatar from "~/atoms/svg/Avatar.svelte";
  import Key from "~/atoms/svg/Key.svelte";
  import NavBar from "~/components/NavBar.svelte";
  import { login } from "~/lib/api/auth";
  import { hashPassword } from "~/lib/crypto";

  let name: string;
  let password: string;
  async function onclick() {
    const authInfo = {
      name,
      password: await hashPassword(password),
    };

    const res = await login(fetch, authInfo);
    if (!res.ok) {
      // TODO: warn that user couldn't log in
      return;
    }
    console.log("successfully logged in as", res.val);
    goto("/home");
  }
</script>

<NavBar title="Log In"></NavBar>
<main>
  <form class="space-y-2">
    <label class="input input-bordered flex items-center gap-2">
      <Avatar />
      <input
        type="text"
        bind:value={name}
        class="grow"
        placeholder="Username"
        required
      />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      <Key />
      <input
        bind:value={password}
        type="password"
        class="grow"
        placeholder="Password"
        required
        minlength="8"
      />
    </label>
    <button on:click={onclick} type="submit" class="btn btn-primary w-full">
      Log In
    </button>
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
