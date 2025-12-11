<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
</script>

<h1 class="page-title">PwdGenT</h1>

<Table>
    <TableHead>
        <TableHeadCell>User Name</TableHeadCell>
        <TableHeadCell>Platform</TableHeadCell>
    </TableHead>
    <TableBody>
        <TableBodyRow onclick={() =>{}}>
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
