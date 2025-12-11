<script lang="ts">
    import {getContext, onDestroy} from "svelte";
    import type {Component} from "svelte";
    import type {MemoryRouter, RouteState} from "./types";

    export let path: string;
    export let component: Component;

    const router = getContext<MemoryRouter>("MEMORY_ROUTER");

    let current: RouteState | null = null;

    const unsubscribe = router.current.subscribe(v => {
        current = v;
    });

    onDestroy(unsubscribe);

    $: active = current?.path === path;
</script>

{#if active}
    <svelte:component this={component}/>
{/if}
