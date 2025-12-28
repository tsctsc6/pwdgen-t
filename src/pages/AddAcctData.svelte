<script lang="ts">
    import {Button, Label, Input, Toggle, ButtonGroup, Clipboard, Tooltip,} from "flowbite-svelte";
    import {
        FloppyDiskAltSolid,
        MinusOutline,
        PlusOutline,
        CheckOutline,
        ClipboardCleanSolid,
        PlaySolid,
        EyeSlashOutline,
        EyeOutline
    }
        from "flowbite-svelte-icons";
    import {invoke} from "@tauri-apps/api/core";
    import {getContext} from "svelte";
    import {type IMemoryRouter, MEMORY_ROUTER} from "../route/types";
    import {message} from "@tauri-apps/plugin-dialog";

    const router = getContext<IMemoryRouter>(MEMORY_ROUTER);

    let userName: string = $state("");
    let platform: string = $state("");
    let remark: string = $state("");
    let nonceOffset: number = $state(0);
    let useUpLetter: boolean = $state(true);
    let useLowLetter: boolean = $state(true);
    let useNumber: boolean = $state(true);
    let useSpecialCharacter: boolean = $state(true);
    let passwordLength: number = $state(15);
    let mainPassword: string = $state("");
    let passwordGenerated: string = $state("");

    let showMainPassword: boolean = $state(false);

    const onSave = async () => {
        let request = {
            user_name: userName,
            platform: platform,
            remark: remark,
            nonce_offset: nonceOffset,
            use_up_letter: useUpLetter,
            use_low_letter: useLowLetter,
            use_number: useNumber,
            use_sp_char: useSpecialCharacter,
            pwd_len: passwordLength,
        };
        try {
            await invoke("create_acct_data", {request});
            router.clear("/home");
        } catch (err) {
            if (typeof err === 'string') {
                await message(err, {title: 'Error', kind: 'error'});
            } else if (err instanceof Error) {
                await message(err.message, {title: 'Error', kind: 'error'});
            }
        }
    }

    const onGenerate = async () => {
        let request = {
            user_name: userName,
            platform: platform,
            nonce_offset: nonceOffset,
            use_up_letter: useUpLetter,
            use_low_letter: useLowLetter,
            use_number: useNumber,
            use_sp_char: useSpecialCharacter,
            pwd_len: passwordLength,
            main_password: mainPassword,
        };
        try {
            passwordGenerated = await invoke("calculate_password", {request});
        } catch (err) {
            if (typeof err === 'string') {
                await message(err, {title: 'Error', kind: 'error'});
            } else if (err instanceof Error) {
                await message(err.message, {title: 'Error', kind: 'error'});
            }
        }
    }
</script>

<div class="flex items-center mb-6 gap-2">
    <h1 class="page-title flex-1">
        Add
    </h1>

    <Button id="submit" pill class="p-2!" onclick={onSave}>
        <FloppyDiskAltSolid class="h-6 w-6"/>
    </Button>
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
    <Label for="nonce-offset" class="mb-2 block">Nonce offset</Label>
    <div class="relative flex max-w-56 items-center">
        <ButtonGroup>
            <Button type="button" id="decrement-button" onclick={() => (nonceOffset -= 1)}>
                <MinusOutline/>
            </Button>
            <Input bind:value={nonceOffset} type="number" id="nonce-offset-input"
                   aria-describedby="helper-text-explanation" placeholder="999" required class="w-32! text-center"/>
            <Button type="button" id="increment-button" onclick={() => (nonceOffset += 1)}>
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
            <Input bind:value={passwordLength} type="number" id="password-length"
                   aria-describedby="helper-text-explanation" placeholder="999" required class="w-32! text-center"/>
            <Button type="button" id="increment-button" onclick={() => (passwordLength += 1)}>
                <PlusOutline/>
            </Button>
        </ButtonGroup>
    </div>
</div>

<div class="flex items-center mb-6 gap-2"></div>

<div class="flex items-center mb-6 gap-2">
    <Input id="main-password" class="pr-14" type={showMainPassword ? 'text' : 'password'} bind:value={mainPassword}
           onkeydown={(e) => {
            if (e.key === 'Enter') {
              onGenerate();
            }
        }}>
        {#snippet right()}
            <Button
                    class="w-5 h-5 bg-transparent hover:bg-transparent border-none
                        focus:bg-transparent active:bg-transparent
                        text-gray-500 dark:text-gray-400"
                    onmousedown={() => (showMainPassword = true)}
                    onmouseup={() => (showMainPassword = false)}
                    onmouseleave={() => (showMainPassword = false)}
                    ontouchstart={() => (showMainPassword = true)}
                    ontouchend={() => (showMainPassword = false)}
            >
                {#if showMainPassword}
                    <EyeSlashOutline/>
                {:else}
                    <EyeOutline/>
                {/if}
            </Button>
        {/snippet}
    </Input>
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
