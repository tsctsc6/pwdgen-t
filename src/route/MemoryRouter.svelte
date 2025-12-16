<script lang="ts">
    import {onMount, onDestroy} from "svelte";
    import type {Component} from "svelte";
    import type {IMemoryRouter} from "./types";

    export let routes: Record<string, Component> = {};
    export let initial = "/init";
    export let router: IMemoryRouter;

    // Completely disable the browser / phone back button
    onMount(() => {
        const handler = (_e: PopStateEvent) => {
        };

        window.addEventListener("popstate", handler);

        // ensure initial route is set from prop (if router provided)
        router?.replace(initial);

        return () => window.removeEventListener("popstate", handler);
    });

    let component: Component | undefined;
    let componentProps: Record<string, unknown> = {};

    const unsubscribe =
        router?.current.subscribe((v) => {
            component = routes[v.path];
            componentProps = v.props ?? {};
        }) ?? (() => {
        });

    onDestroy(unsubscribe);
</script>

{#if component}
    <svelte:component this={component} {...componentProps}/>
{/if}
