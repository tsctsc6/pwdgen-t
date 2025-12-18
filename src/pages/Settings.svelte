<script lang="ts">
    import {Label, Button} from "flowbite-svelte";
    import {invoke} from "@tauri-apps/api/core";

    const onBackup = async () => {
        await invoke("backup");
    }

    const onRestore = async () => {
        await invoke("restore");
    }
</script>

<div class="mb-6">
    <h1 class="page-title">
        Settings
    </h1>
</div>

<div class="items-center mb-6 gap-2">
    <Button id="backup" class="p-2!" onclick={onBackup}>
        Backup
    </Button>
    <Button id="backup" class="p-2!" onclick={onRestore}>
        Restore
    </Button>
</div>

<div class="mb-6">
    <Label id="version">Version 0.1.0-beta</Label>
</div>
