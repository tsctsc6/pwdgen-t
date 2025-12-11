<script lang="ts">
    import {invoke} from '@tauri-apps/api/core';
    import {onMount} from "svelte";
    import {useRouter} from "../route/useRouter";

    const {replace} = useRouter();

    onMount(async () => {
        try {
            console.log("Initializing...");
            await invoke('init_migrate');
        } catch (err) {
            console.log(err);
        } finally {
            replace("/home")
        }
    })
</script>

<p>
    Migrating...
</p>
