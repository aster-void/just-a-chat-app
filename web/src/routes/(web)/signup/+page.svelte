<script lang="ts">
  import { run } from "svelte/legacy";

  import { goto } from "$app/navigation";
  import { validator } from "@felte/validator-zod";
  import { createForm } from "felte";
  import Avatar from "~/atoms/svg/Avatar.svelte";
  import Key from "~/atoms/svg/Key.svelte";
  import NavBar from "~/components/NavBar.svelte";
  import { pushToast } from "~/components/toast/toast.store";
  import { signup } from "~/lib/api/auth";
  import { isAvailable } from "~/lib/api/user";
  import { InitUserSchema } from "~/lib/schema";
  import type { InitUser } from "~/lib/types";

  let username: string = $state("");

  const { form, errors } = createForm<InitUser>({
    extend: [validator({ schema: InitUserSchema })],
    async onSubmit(values) {
      try {
        await signup(fetch, values);
        goto("/home");
      } catch (err: unknown) {
        pushToast((err as Error).message, "error", 2000);
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

  let disabled = $state(true);
  run(() => {
    disabled = !($errors.name === null && $errors.rawPassword === null);
  });
</script>

<main>
  <form use:form class="space-y-2">
    <label
      class="input input-bordered flex items-center gap-2
      {$errors.name ? 'input-error' : ''}"
    >
      <Avatar />
      <input
        name="name"
        bind:value={username}
        type="text"
        placeholder="Username"
        required
        minlength="3"
        maxlength="255"
      />
    </label>
    {#if $errors.name}
      <span class="text-red-400 error">{$errors.name.join(", ")}</span>
    {/if}
    <label
      class="input input-bordered flex items-center gap-2
        {$errors.rawPassword ? 'input-error' : ''}"
    >
      <Key />
      <input
        name="rawPassword"
        type="password"
        placeholder="Password"
        required
        minlength="8"
        maxlength="255"
      />
    </label>
    {#if $errors.rawPassword}
      <span class="text-red-400 error">{$errors.rawPassword.join(", ")}</span>
    {/if}
    <button type="submit" class="btn btn-primary w-full" {disabled}>
      Create Account
    </button>
  </form>
</main>

<style>
  span.error {
    flex-wrap: wrap;
  }
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
