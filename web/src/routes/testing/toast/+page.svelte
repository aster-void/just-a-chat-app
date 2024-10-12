<script lang="ts">
  import { pushToast } from "../../../components/toast/toast.store";
</script>

<button on:click={() => pushToast("Default", "default", 2000)}>Default</button>
<button on:click={() => pushToast("Success", "success", 2000)}>Success</button>
<button on:click={() => pushToast("Info", "info", 2000)}>Info</button>
<button on:click={() => pushToast("Warning", "warning", 2000)}>Warning</button>
<button on:click={() => pushToast("Error", "error", 2000)}>Error</button>
