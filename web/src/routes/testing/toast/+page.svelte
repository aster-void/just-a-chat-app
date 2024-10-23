<script lang="ts">
import { pushToast } from "../../../components/toast/toast.store";
</script>

<button onclick={() => pushToast("Default", "default", 2000)}>Default</button>
<button onclick={() => pushToast("Success", "success", 2000)}>Success</button>
<button onclick={() => pushToast("Info", "info", 2000)}>Info</button>
<button onclick={() => pushToast("Warning", "warning", 2000)}>Warning</button>
<button onclick={() => pushToast("Error", "error", 2000)}>Error</button>
