<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {getContext, onMount} from "svelte";
    import {Spinner} from "flowbite-svelte";
    import {type IMemoryRouter, MEMORY_ROUTER} from "../route/types";

    const router = getContext<IMemoryRouter>(MEMORY_ROUTER);

    onMount(async () => {
        try {
            console.log("Initializing...");
            await invoke("init_migrate");
        } catch (err) {
            console.log(err);
        } finally {
            router.replace("/home");
        }
    });
</script>

<Spinner/>
