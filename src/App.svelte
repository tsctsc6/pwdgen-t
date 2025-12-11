<script lang="ts">
    import MemoryRouter from "./route/MemoryRouter.svelte";
    import Route from "./route/Route.svelte";
    import Init from "./pages/Init.svelte";
    import Home from "./pages/Home.svelte";
    import Settings from "./pages/Settings.svelte";
    import MainPage from "./MainPage.svelte";
    import './app.css';
    import AddAcctData from "./pages/AddAcctData.svelte";
</script>

<MemoryRouter>
    <MainPage>
        <Route path="/init" component={Init}/>
        <Route path="/home" component={Home}/>
        <Route path="/settings" component={Settings}/>
        <Route path="/add-acct-data" component={AddAcctData}/>
    </MainPage>
</MemoryRouter>
