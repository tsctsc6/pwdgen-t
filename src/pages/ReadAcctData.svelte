<script lang="ts">
    import {Button, Label, Input, Clipboard, Tooltip, Spinner} from "flowbite-svelte";
    import {CaretLeftSolid, CheckOutline, ClipboardCleanSolid, PlaySolid}
        from "flowbite-svelte-icons";
    import {invoke} from "@tauri-apps/api/core";
    import {getContext, onMount} from "svelte";
    import {type MemoryRouter, SingletonKey} from "../route/types";
    import type {read_acct_data_result} from "../models/read_acct_data_result";

    const router = getContext<MemoryRouter>(SingletonKey);

    let {id} = $props();

    let acctData: read_acct_data_result | null = $state(null);

    let mainPassword: string = $state("");
    let passwordGenerated: string = $state("");

    onMount(async () => {
        console.log(id);
        acctData = await invoke("read_acct_data", {id});
    });

    const onBack = () => {
        router.back();
    }

    const onGenerate = () => {
        console.log("onGenerate");
    }
</script>

<div class="flex items-center mb-6 gap-2">
    <Button id="submit" pill class="p-2!" onclick={onBack}>
        <CaretLeftSolid class="h-6 w-6"/>
    </Button>

    <h1 class="page-title flex-1">
        Details
    </h1>
</div>

{#if acctData === null}
    <Spinner/>
{:else}
    <div class="flex items-center mb-6 gap-2">
        <Label for="user-name" class="mb-2 block">User name</Label>
        <Label id="user-name">{acctData.user_name}</Label>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="platform" class="mb-2 block">Platform</Label>
        <Label id="platform">{acctData.platform}</Label>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="remark" class="mb-2 block">Remark</Label>
        <Label id="remark">{acctData.remark}</Label>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="skip-count" class="mb-2 block">Skip count</Label>
        <Label id="skip-count">{acctData.skip_count}</Label>
    </div>
    <div class="mb-6">
        <Label for="use-up-letter" class="mb-2 block">Use up letter</Label>
        <Label id="use-up-letter">{acctData.use_up_letter}</Label>
    </div>
    <div class="mb-6">
        <Label for="use-low-letter" class="mb-2 block">Use low letter</Label>
        <Label id="use-low-letter">{acctData.use_low_letter}</Label>
    </div>
    <div class="mb-6">
        <Label for="use-number" class="mb-2 block">Use number</Label>
        <Label id="use-number">{acctData.use_number}</Label>
    </div>
    <div class="mb-6">
        <Label for="use-special-character" class="mb-2 block">Use special character</Label>
        <Label id="use-special-character">{acctData.use_sp_char}</Label>
    </div>
    <div class="mb-6">
        <Label for="use-up-letter" class="mb-2 block">Password length</Label>
        <Label id="use-up-letter">{acctData.pwd_len}</Label>
    </div>

    <div class="flex items-center mb-6 gap-2"></div>

    <div class="flex items-center mb-6 gap-2">
        <Input id="default-input" type="password" bind:value={mainPassword}/>
        <Button id="generate" pill class="p-2!" onclick={onGenerate}>
            <PlaySolid class="h-6 w-6"/>
        </Button>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="password-generated" class="mb-2 block flex-1">{passwordGenerated}</Label>
        <Clipboard color="primary" bind:value={passwordGenerated}>
            {#snippet children(success)}
                <Tooltip class="whitespace-nowrap">{success ? "Copied" : "Copy to clipboard"}</Tooltip>
                {#if success}
                    <CheckOutline/>
                {:else}
                    <ClipboardCleanSolid/>
                {/if}
            {/snippet}
        </Clipboard>
    </div>
{/if}