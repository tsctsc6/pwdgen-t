<script lang="ts">
    import {
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
        Button,
        PaginationNav,
        Search
    } from "flowbite-svelte";
    import {SearchSolid, ArrowLeftOutline, ArrowRightOutline} from "flowbite-svelte-icons";
    import {Spinner} from "flowbite-svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {getContext, onMount} from "svelte";
    import type {page_content_type, read_all_acct_data_result} from "../models/read_all_acct_data_result";
    import {type IMemoryRouter, MEMORY_ROUTER} from "../route/types";
    import {message} from "@tauri-apps/plugin-dialog";

    const router = getContext<IMemoryRouter>(MEMORY_ROUTER);

    let searchTerm = $state("");
    let currentPage = $state(1);
    let pageSize = $state(10);
    let totalPages = $state(3);
    let pageContent: page_content_type[] | null = $state(null);

    const onSearch = async () => {
        await getData();
    }

    const onPageChange = async (page: number) => {
        currentPage = page;
        await getData();
    }

    const getData = async () => {
        pageContent = null;
        let pageIndex = currentPage - 1;
        let result: read_all_acct_data_result = {page_count: 0, page_content: []}
        try {
            result = await invoke("read_all_acct_data", {
                searchTerm,
                pageIndex,
                pageSize
            });
        } catch (err) {
            if (typeof err === 'string') {
                await message(err, {title: 'Error', kind: 'error'});
            } else if (err instanceof Error) {
                await message(err.message, {title: 'Error', kind: 'error'});
            }
        }
        if (result.page_count === 0) {
            totalPages = 1;
        } else {
            totalPages = result.page_count;
        }
        pageContent = result.page_content;
    }

    const onClickRow = (id: number) => {
        router.push("/read-acct-data", {id: id});
    }

    onMount(async () => {
        await getData();
    })
</script>

<div class="mb-6">
    <h1 class="page-title">PwdGenT</h1>
</div>

<div class="mb-6">
    <Search id="search-input" bind:value={searchTerm}>
        <Button id="search-button" class="me-1" onclick={onSearch}>
            <SearchSolid class="h-6 w-6"/>
        </Button>
    </Search>
</div>

{#if pageContent === null}
    <Spinner/>
{:else}
    <Table>
        <TableHead>
            <TableHeadCell>Id</TableHeadCell>
            <TableHeadCell>User Name</TableHeadCell>
            <TableHeadCell>Platform</TableHeadCell>
        </TableHead>
        <TableBody>
            {#each pageContent as item}
                <TableBodyRow onclick={() =>{onClickRow(item.id)}}>
                    <TableBodyCell>{item.id}</TableBodyCell>
                    <TableBodyCell>{item.user_name}</TableBodyCell>
                    <TableBodyCell>{item.platform}</TableBodyCell>
                </TableBodyRow>

            {/each}
        </TableBody>
    </Table>
{/if}

<PaginationNav {currentPage} {totalPages} {onPageChange}>
    {#snippet prevContent()}
        <span class="sr-only">Previous</span>
        <ArrowLeftOutline class="h-5 w-5"/>
    {/snippet}
    {#snippet nextContent()}
        <span class="sr-only">Next</span>
        <ArrowRightOutline class="h-5 w-5"/>
    {/snippet}
</PaginationNav>
