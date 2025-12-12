<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Button, PaginationNav,Search } from "flowbite-svelte";
    import { SearchSolid,ArrowLeftOutline,ArrowRightOutline } from "flowbite-svelte-icons";
    import {invoke} from "@tauri-apps/api/core";

    let searchTerm = $state("");
    let currentPage = $state(1);
    let pageSize = $state(10);
    let totalPages = 3;

    const onSearch = async () => {
        console.log(searchTerm);
        await getData();
    }

    const onPageChange = async (page: number) => {
        currentPage = page;
        console.log(currentPage);
        await getData();
    }

    const getData = async () => {
        let pageIndex = currentPage - 1;
        let x = await invoke("read_all_acct_data", {searchTerm, pageIndex, pageSize});
        console.log(x);
    }

    const onClickRow = (tableIndex: number) => {
        console.log(tableIndex);
    }
</script>

<div class="mb-6">
    <h1 class="page-title">PwdGenT</h1>
</div>

<div class="mb-6">
    <Search id="search-input" bind:value={searchTerm}>
        <Button id="search-button" class="me-1" onclick={onSearch}>
            <SearchSolid class="h-6 w-6" />
        </Button>
    </Search>
</div>

<Table>
    <TableHead>
        <TableHeadCell>User Name</TableHeadCell>
        <TableHeadCell>Platform</TableHeadCell>
    </TableHead>
    <TableBody>
        <TableBodyRow onclick={() =>{onClickRow(0)}}>
            <TableBodyCell>Apple MacBook Pro 17</TableBodyCell>
            <TableBodyCell>Silver</TableBodyCell>
        </TableBodyRow>
        <TableBodyRow>
            <TableBodyCell>Microsoft Surface Pro</TableBodyCell>
            <TableBodyCell>White</TableBodyCell>
        </TableBodyRow>
        <TableBodyRow>
            <TableBodyCell>Magic Mouse 2</TableBodyCell>
            <TableBodyCell>Black</TableBodyCell>
        </TableBodyRow>
    </TableBody>
</Table>

<PaginationNav {currentPage} {totalPages} {onPageChange}>
    {#snippet prevContent()}
        <span class="sr-only">Previous</span>
        <ArrowLeftOutline class="h-5 w-5" />
    {/snippet}
    {#snippet nextContent()}
        <span class="sr-only">Next</span>
        <ArrowRightOutline class="h-5 w-5" />
    {/snippet}
</PaginationNav>
