<script lang="ts">
  import { toasts } from "./toast.store";
</script>

{#if $toasts.length > 0}
  <div class="toast toast-top toast-start">
    {#each $toasts as toast (toast.id)}
      {#if toast.kind === "success"}
        <div class="alert alert-success">
          <span>{toast.message}</span>
        </div>
      {:else if toast.kind === "info"}
        <div class="alert alert-info">
          <span>{toast.message}</span>
        </div>
      {:else if toast.kind === "warning"}
        <div class="alert alert-warning">
          <span>{toast.message}</span>
        </div>
      {:else if toast.kind === "error"}
        <div class="alert alert-error">
          <span>{toast.message}</span>
        </div>
      {/if}
    {/each}
  </div>
{/if}
