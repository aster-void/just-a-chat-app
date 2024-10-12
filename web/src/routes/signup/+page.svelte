<script lang="ts">
  import NavBar from "~/components/NavBar.svelte";
  import Avatar from "~/atoms/svg/Avatar.svelte";
  import Key from "~/atoms/svg/Key.svelte";
  import { signup } from "~/lib/api/auth";
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import { InitUserSchema } from "~/lib/schema";
  import { isAvailable } from "~/lib/api/user";
  import { goto } from "$app/navigation";
  import type { InitUser } from "~/lib/types";
  import { pushToast } from "~/components/toast/toast.store";

  let username: string;

  const { form, errors } = createForm<InitUser>({
    extend: [validator({ schema: InitUserSchema })],
    async onSubmit(values) {
      try {
        await signup(fetch, values);
        goto("/home");
      } catch (err: any) {
        pushToast(err.message, "error", 2000);
      }
    },
    debounced: {
      timeout: 300,
      async validate(values) {
        try {
          return {
            name: (await isAvailable(fetch, values.name))
              ? null
              : "Name is already taken",
            rawPassword: null,
          };
        } catch (err) {
          return {
            name: "Failed to check username availability",
            rawPassword: null,
          };
        }
      },
    },
  });

  let disabled = true;
  $: disabled = !($errors.name === null && $errors.rawPassword === null);
</script>

<NavBar title="Sign Up"></NavBar>
<main>
  <pre>{JSON.stringify($errors, null, 2)}</pre>
  <form use:form class="space-y-2">
    <label class="input input-bordered flex items-center gap-2">
      <Avatar />
      <input
        name="name"
        bind:value={username}
        type="text"
        class="grow input-bordered {$errors.name ? 'input-error' : ''}"
        placeholder="Username"
        required
        minlength="3"
        maxlength="255"
      />
    </label>
    {#if $errors.name}
      <span class="input-error">{$errors.name.join(", ")}</span>
    {/if}
    <label class="input input-bordered flex items-center gap-2">
      <Key />
      <input
        name="rawPassword"
        type="password"
        class="grow"
        placeholder="Password"
        required
        minlength="8"
        maxlength="255"
      />
    </label>
    {#if $errors.rawPassword}
      <span class="red">{$errors.rawPassword.join(", ")}</span>
    {/if}
    <button type="submit" class="btn btn-primary w-full" {disabled}>
      Create Account
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
