<script lang="ts">
    import {Button, Label, Input, Toggle, ButtonGroup, Clipboard, Tooltip, Spinner} from "flowbite-svelte";
    import {
        MinusOutline, PlusOutline, CheckOutline, ClipboardCleanSolid, PlaySolid, FloppyDiskAltSolid,
        CaretLeftSolid
    }
        from "flowbite-svelte-icons";
    import {invoke} from "@tauri-apps/api/core";
    import {getContext, onMount} from "svelte";
    import {type IMemoryRouter, SingletonKey} from "../route/types";
    import type {read_acct_data_result} from "../models/read_acct_data_result";

    const router = getContext<IMemoryRouter>(SingletonKey);

    let {id} = $props();

    let acctData: read_acct_data_result | null = $state(null);

    let userName: string = $state("");
    let platform: string = $state("");
    let remark: string = $state("");
    let skipCount: number = $state(0);
    let useUpLetter: boolean = $state(true);
    let useLowLetter: boolean = $state(true);
    let useNumber: boolean = $state(true);
    let useSpecialCharacter: boolean = $state(true);
    let passwordLength: number = $state(15);
    let mainPassword: string = $state("");
    let passwordGenerated: string = $state("");

    onMount(async () => {
        console.log(id);
        acctData = await invoke("read_acct_data", {id});
        if (acctData === null) {
            return;
        }
        userName = acctData.user_name;
        platform = acctData.platform;
        remark = acctData.remark;
        skipCount = acctData.skip_count;
        useUpLetter = acctData.use_up_letter;
        useLowLetter = acctData.use_low_letter;
        useNumber = acctData.use_number;
        useSpecialCharacter = acctData.use_sp_char;
        passwordLength = acctData.pwd_len;
    });

    const onSave = async () => {
        if (acctData === null) {
            return;
        }
        let request = {
            id: acctData.id,
            user_name: userName,
            platform: platform,
            remark: remark,
            skip_count: skipCount,
            use_up_letter: useUpLetter,
            use_low_letter: useLowLetter,
            use_number: useNumber,
            use_sp_char: useSpecialCharacter,
            pwd_len: passwordLength,
        };
        await invoke("update_acct_data", {request});
        router.back();
    }

    const onGenerate = async () => {
        let request = {
            user_name: userName,
            platform: platform,
            skip_count: skipCount,
            use_up_letter: useUpLetter,
            use_low_letter: useLowLetter,
            use_number: useNumber,
            use_sp_char: useSpecialCharacter,
            pwd_len: passwordLength,
            main_password: mainPassword,
        };
        passwordGenerated = await invoke("calculate_password", {request});
    }

    const onBack = () => {
        router.back();
    }
</script>

<div class="flex items-center mb-6 gap-3">
    <Button id="back-button" pill class="p-2!" onclick={onBack}>
        <CaretLeftSolid class="h-6 w-6"/>
    </Button>

    <h1 class="page-title flex-1">
        Edit
    </h1>

    <Button id="edit-button" pill class="p-2!" onclick={onSave}>
        <FloppyDiskAltSolid class="h-6 w-6"/>
    </Button>
</div>

{#if acctData === null}
    <Spinner/>
{:else}
    <div class="flex items-center mb-6 gap-2">
        <Label for="id" class="mb-2 block">Id</Label>
        <Input readonly disabled id="id" bind:value={acctData.id}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="user-name" class="mb-2 block">User name</Label>
        <Input id="user-name" placeholder="User name" bind:value={userName}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="platform" class="mb-2 block">Platform</Label>
        <Input id="platform" placeholder="Platform" bind:value={platform}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="remark" class="mb-2 block">Remark</Label>
        <Input id="remark" placeholder="Remark" bind:value={remark}/>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="skip-count" class="mb-2 block">Skip count</Label>
        <div class="relative flex max-w-56 items-center">
            <ButtonGroup>
                <Button type="button" id="decrement-button" onclick={() => (skipCount -= 1)}>
                    <MinusOutline/>
                </Button>
                <Input bind:value={skipCount} type="number" id="skip-count-input"
                       aria-describedby="helper-text-explanation" placeholder="999" required class="w-32! text-center"/>
                <Button type="button" id="increment-button" onclick={() => (skipCount += 1)}>
                    <PlusOutline/>
                </Button>
            </ButtonGroup>
        </div>
    </div>
    <div class="mb-6">
        <Toggle bind:checked={useUpLetter}>Use up letter</Toggle>
    </div>
    <div class="mb-6">
        <Toggle bind:checked={useLowLetter}>Use low letter</Toggle>
    </div>
    <div class="mb-6">
        <Toggle bind:checked={useNumber}>Use number</Toggle>
    </div>
    <div class="mb-6">
        <Toggle bind:checked={useSpecialCharacter}>Use special character</Toggle>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="password-length" class="mb-2 block">Password length</Label>
        <div class="relative flex max-w-56 items-center">
            <ButtonGroup>
                <Button type="button" id="decrement-button" onclick={() => (passwordLength -= 1)}>
                    <MinusOutline/>
                </Button>
                <Input bind:value={passwordLength} type="number" id="password-length-input"
                       aria-describedby="helper-text-explanation" placeholder="999" required class="w-32! text-center"/>
                <Button type="button" id="increment-button" onclick={() => (passwordLength += 1)}>
                    <PlusOutline/>
                </Button>
            </ButtonGroup>
        </div>
    </div>
    <div class="flex items-center mb-6 gap-2">
        <Label for="updated-at" class="mb-2 block">Updated at</Label>
        <Input readonly disabled id="updated-at" bind:value={acctData.updated_at}/>
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
