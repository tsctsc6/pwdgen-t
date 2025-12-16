<script lang="ts">
    import {Button, Label, Input, Clipboard, Tooltip, Spinner, ButtonGroup, Toggle} from "flowbite-svelte";
    import {CaretLeftSolid, CheckOutline, ClipboardCleanSolid, PlaySolid, EditSolid, MinusOutline, PlusOutline}
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

    const onEdit = () => {
        if (acctData === null) {
            return;
        }
        router.push("/edit-acct-data", {id: acctData.id});
    }

    const onGenerate = () => {
        console.log("onGenerate");
    }
</script>

<div class="flex items-center mb-6 gap-3">
    <Button id="back-button" pill class="p-2!" onclick={onBack}>
        <CaretLeftSolid class="h-6 w-6"/>
    </Button>

    <h1 class="page-title flex-1">
        Details
    </h1>

    <Button id="edit-button" pill class="p-2!" onclick={onEdit}>
        <EditSolid class="h-6 w-6"/>
    </Button>
</div>

{#if acctData === null}
    <Spinner/>
{:else}
    <div class="flex items-center mb-6 gap-2">
        <Label for="id" class="mb-2 block">Id</Label>
        <Input readonly id="id" bind:value={acctData.id}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="user-name" class="mb-2 block">User name</Label>
        <Input readonly id="user-name" bind:value={acctData.user_name}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="platform" class="mb-2 block">Platform</Label>
        <Input readonly id="platform" bind:value={acctData.platform}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="remark" class="mb-2 block">Remark</Label>
        <Input readonly id="remark" bind:value={acctData.remark}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="skip-count" class="mb-2 block">Skip count</Label>
        <div class="relative flex max-w-56 items-center">
            <ButtonGroup>
                <Button disabled type="button" id="decrement-button">
                    <MinusOutline/>
                </Button>
                <Input readonly bind:value={acctData.skip_count} type="number" id="skip-count-input"
                       aria-describedby="helper-text-explanation" placeholder="999" required class="w-32! text-center"/>
                <Button disabled type="button" id="increment-button">
                    <PlusOutline/>
                </Button>
            </ButtonGroup>
        </div>
    </div>
    <div class="mb-6">
        <Toggle disabled bind:checked={acctData.use_up_letter}>Use up letter</Toggle>
    </div>
    <div class="mb-6">
        <Toggle disabled bind:checked={acctData.use_low_letter}>Use low letter</Toggle>
    </div>
    <div class="mb-6">
        <Toggle disabled bind:checked={acctData.use_number}>Use number</Toggle>
    </div>
    <div class="mb-6">
        <Toggle disabled bind:checked={acctData.use_sp_char}>Use special character</Toggle>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="password-length" class="mb-2 block">Password length</Label>
        <div class="relative flex max-w-56 items-center">
            <ButtonGroup>
                <Button disabled type="button" id="decrement-button">
                    <MinusOutline/>
                </Button>
                <Input readonly bind:value={acctData.pwd_len} type="number" id="password-length-input"
                       aria-describedby="helper-text-explanation" placeholder="999" required class="w-32! text-center"/>
                <Button disabled type="button" id="increment-button">
                    <PlusOutline/>
                </Button>
            </ButtonGroup>
        </div>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="updated-at" class="mb-2 block">Updated at</Label>
        <Input readonly id="updated-at" bind:value={acctData.updated_at}/>
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
