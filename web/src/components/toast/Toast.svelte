<script lang="ts">
import { fade } from "svelte/transition";
import { toasts } from "./toast.store";
</script>

<!-- ALLOW TAILWIND TO RECOGNIZE DYNAMIC ALERT-{} CLASS, DO NOT DELETE THIS ELEMENT -->
<div class="alert-success alert-info alert-warning alert-error" hidden />

<div role="alert" class="toast toast-bottom toast-start">
  {#each $toasts as toast (toast.id)}
    <!-- TODO: animate this up down -->
    <div
      out:fade={{ duration: 150 }}
      in:fade={{ duration: 150 }}
      class="alert alert-{toast.kind}"
    >
      <span>{toast.message}</span>
    </div>
  {/each}
</div>

<style>
</style>
