<script>
    import { BottomNav, BottomNavItem, Tooltip } from "flowbite-svelte";
    import { HomeSolid, AdjustmentsVerticalOutline, PlusOutline } from "flowbite-svelte-icons";
    import {useRouter} from "./route/useRouter";
    const {clear} = useRouter();
</script>

<slot/>

<BottomNav position="absolute" navType="application" classes={{ inner: "grid-cols-3" }}>
    <BottomNavItem btnName="Home" appBtnPosition="left" onclick={() => clear("/home")}>
        <HomeSolid class="group-hover:text-primary-600 dark:group-hover:text-primary-500 mb-1 h-6 w-6 text-gray-500 dark:text-gray-400" />
    </BottomNavItem>
    <Tooltip arrow={false}>Home</Tooltip>
    <div class="flex items-center justify-center">
        <BottomNavItem
                btnName="Create new item"
                appBtnPosition="middle"
                class="plus-button bg-primary-600 hover:bg-primary-700 group focus:ring-primary-300 dark:focus:ring-primary-800 inline-flex h-10 w-10 items-center justify-center rounded-full font-medium focus:ring-4 focus:outline-hidden"
                style="background-color: #EF562F;"
                onclick={() => clear("/add-acct-data")}
        >
            <PlusOutline class="text-white" />
        </BottomNavItem>
        <Tooltip arrow={false}>Create new item</Tooltip>
    </div>
    <BottomNavItem btnName="Settings" appBtnPosition="right" onclick={() => clear("/settings")}>
        <AdjustmentsVerticalOutline class="group-hover:text-primary-600 dark:group-hover:text-primary-500 mb-1 h-6 w-6 text-gray-500 dark:text-gray-400" />
    </BottomNavItem>
    <Tooltip arrow={false}>Settings</Tooltip>
</BottomNav>
