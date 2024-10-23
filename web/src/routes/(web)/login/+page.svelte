<script lang="ts">
import { goto } from "$app/navigation";
import Avatar from "~/atoms/svg/Avatar.svelte";
import Key from "~/atoms/svg/Key.svelte";
import { pushToast } from "~/components/toast/toast.store";
import { login } from "~/lib/api/auth";
import { leftOff } from "~/lib/api/internal/token-store";

let name: string;
let password: string;
async function onclick() {
	const authInfo = {
		name,
		rawPassword: password,
	};

	const res = await login(fetch, authInfo);
	if (!res.ok) {
		console.error(res.err);
		pushToast("Sorry, failed to log in", "error", 2000);
		return;
	}
	console.log("successfully logged in as", res.val);
	goto($leftOff ?? "/home");
}
</script>

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
