<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { Component } from "svelte";
  import type { MemoryRouter as MemoryRouterType } from "./types";

  export let routes: Record<string, Component> = {};
  export let initial = "/init";
  export let router: MemoryRouterType;

  // Completely disable the browser / phone back button
  onMount(() => {
    const handler = (_e: PopStateEvent) => {};

    window.addEventListener("popstate", handler);

    // ensure initial route is set from prop (if router provided)
    router?.replace(initial);

    return () => window.removeEventListener("popstate", handler);
  });

  let component: Component | undefined;
  const unsubscribe =
    router?.current.subscribe((v) => {
      component = routes[v.path];
    }) ?? (() => {});

  onDestroy(unsubscribe);
</script>

{#if component}
  <svelte:component this={component} />
{/if}
