<script lang="ts">
  import NavBar from "~/components/NavBar.svelte";
  import Avatar from "~/atoms/svg/Avatar.svelte";
  import Key from "~/atoms/svg/Key.svelte";
  import { signup } from "~/lib/api/auth";
  import { createForm } from "felte";
  import { InitUserSchema, UserNameSchema, UserSchema } from "~/lib/schema";
  import { hashPassword } from "~/lib/crypto";
  import { isAvailable } from "~/lib/api/user";
  import { writable, type Writable } from "svelte/store";
  import { goto } from "$app/navigation";

  const { form, errors } = createForm({
    async onSubmit(values: unknown) {
      try {
        const init = InitUserSchema.parse(values);
        init.password = await hashPassword(init.password);
        await signup(fetch, init);
        goto("/home"); // todo
      } catch (err) {
        console.error(err);
      }
    },
    async validate(values: unknown) {
      const result = InitUserSchema.safeParse(values);
      if (result.success) return {};
      console.log(result.error.formErrors.fieldErrors);
      return {
        ...result.error.formErrors.fieldErrors,
      };
    },
  });

  let nameIsAvailable: Writable<"invalid" | "ok" | "no" | "loading" | Error> =
    writable("invalid");
  let name: string;
  function onChangeName(newName: string) {
    if (!UserNameSchema.safeParse(newName).success) {
      return nameIsAvailable.set("invalid");
    }
    nameIsAvailable.set("loading");
    isAvailable(fetch, newName)
      .then((v) => {
        if (v) {
          nameIsAvailable.set("ok");
        } else {
          nameIsAvailable.set("no");
        }
      })
      .catch((err) => {
        nameIsAvailable.set(err as Error);
      });
  }
</script>

<NavBar title="Sign Up"></NavBar>
<main>
  <form use:form class="space-y-2">
    <label class="input input-bordered flex items-center gap-2">
      <Avatar />
      <input
        name="name"
        bind:value={name}
        on:input={() => onChangeName(name)}
        type="text"
        class="grow"
        placeholder="Username"
        required
        minlength="3"
        maxlength="255"
      />
    </label>
    {#if $nameIsAvailable === "invalid"}
      <span></span>
    {:else if $nameIsAvailable === "loading"}
      <span> Checking name availability...</span>
    {:else if $nameIsAvailable === "ok"}
      <span>Name is available</span>
    {:else if $nameIsAvailable === "no"}
      <span>Name is not available</span>
    {:else}
      <span>Failed to check name availability: {$nameIsAvailable.message}</span>
    {/if}
    <label class="input input-bordered flex items-center gap-2">
      <Key />
      <input
        name="password"
        type="password"
        class="grow"
        placeholder="Password"
        required
        minlength="8"
        maxlength="255"
      />
    </label>
    <button
      type="submit"
      class="btn btn-primary w-full"
      disabled={!($nameIsAvailable === "ok" && name.length >= 3)}
    >
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
