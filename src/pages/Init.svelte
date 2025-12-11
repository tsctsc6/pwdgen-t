<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getContext, onMount } from "svelte";
  import type { MemoryRouter as MemoryRouterType } from "../route/types";

  const router = getContext<MemoryRouterType>("MEMORY_ROUTER");

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

<p>Migrating...</p>
