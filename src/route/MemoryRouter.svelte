<script lang="ts">
    import {setContext, onMount} from "svelte";
    import {router} from "./memory-router";
    import type {MemoryRouter as MemoryRouterType} from "./types";

    setContext<MemoryRouterType>("MEMORY_ROUTER", router);

    // Completely disable the browser / phone back button
    onMount(() => {
        const handler = (_e: PopStateEvent) => {
        };

        window.addEventListener("popstate", handler);

        return () => window.removeEventListener("popstate", handler);
    });
</script>

<slot/>
