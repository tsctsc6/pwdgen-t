<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {getContext, onMount} from "svelte";
    import {Spinner} from "flowbite-svelte";
    import {type IMemoryRouter, MEMORY_ROUTER} from "../route/types";
    import {message} from "@tauri-apps/plugin-dialog";

    const router = getContext<IMemoryRouter>(MEMORY_ROUTER);

    onMount(async () => {
        try {
            await invoke("init_migrate");
            router.clear("/home");
        } catch (err) {
            if (typeof err === 'string') {
                await message(err, {title: 'Error', kind: 'error'});
            } else if (err instanceof Error) {
                await message(err.message, {title: 'Error', kind: 'error'});
            }
        }
    });
</script>

<Spinner/>
