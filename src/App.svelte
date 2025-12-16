<script lang="ts">
    import MemoryRouter from "./route/MemoryRouter.svelte";
    import {setContext, type Component} from "svelte";
    import {createMemoryRouter} from "./route/memory-router";
    import {
        type IMemoryRouter,
        SingletonKey,
    } from "./route/types";
    import Init from "./pages/Init.svelte";
    import Home from "./pages/Home.svelte";
    import Settings from "./pages/Settings.svelte";
    import "./app.css";
    import AddAcctData from "./pages/AddAcctData.svelte";
    import {BottomNav, BottomNavItem, Tooltip} from "flowbite-svelte";

    import {
        HomeSolid,
        AdjustmentsVerticalOutline,
        PlusOutline,
    } from "flowbite-svelte-icons";
    import ReadAcctData from "./pages/ReadAcctData.svelte";
    import EditAcctData from "./pages/EditAcctData.svelte";

    const routes = {
        "/init": Init,
        "/home": Home,
        "/settings": Settings,
        "/add-acct-data": AddAcctData,
        "/read-acct-data": ReadAcctData,
        "/edit-acct-data": EditAcctData,
    } as unknown as Record<string, Component>;

    const router = createMemoryRouter();

    setContext<IMemoryRouter>(SingletonKey, router);
</script>

<MemoryRouter {routes} {router} initial="/init"/>

<div class="flex items-center mb-6"></div>
<div class="flex items-center mb-6"></div>
<div class="flex items-center mb-6"></div>

<BottomNav
        position="fixed"
        navType="application"
        classes={{ inner: "grid-cols-3" }}
>
    <BottomNavItem
            btnName="Home"
            appBtnPosition="left"
            onclick={() => router.clear("/home")}
    >
        <HomeSolid
                class="group-hover:text-primary-600 dark:group-hover:text-primary-500 mb-1 h-6 w-6 text-gray-500 dark:text-gray-400"
        />
    </BottomNavItem>
    <Tooltip arrow={false}>Home</Tooltip>
    <div class="flex items-center justify-center">
        <BottomNavItem
                btnName="Create new item"
                appBtnPosition="middle"
                class="plus-button bg-primary-600 hover:bg-primary-700 group focus:ring-primary-300 dark:focus:ring-primary-800 inline-flex h-10 w-10 items-center justify-center rounded-full font-medium focus:ring-4 focus:outline-hidden"
                onclick={() => router.clear("/add-acct-data")}
        >
            <PlusOutline class="text-white"/>
        </BottomNavItem>
        <Tooltip arrow={false}>Create new item</Tooltip>
    </div>
    <BottomNavItem
            btnName="Settings"
            appBtnPosition="right"
            onclick={() => router.clear("/settings")}
    >
        <AdjustmentsVerticalOutline
                class="group-hover:text-primary-600 dark:group-hover:text-primary-500 mb-1 h-6 w-6 text-gray-500 dark:text-gray-400"
        />
    </BottomNavItem>
    <Tooltip arrow={false}>Settings</Tooltip>
</BottomNav>
