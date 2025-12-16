<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {getContext, onMount} from "svelte";
    import {Spinner} from "flowbite-svelte";
    import {type MemoryRouter, SingletonKey} from "../route/types";

    const router = getContext<MemoryRouter>(SingletonKey);

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
