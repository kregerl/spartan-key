<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import SelectVault from "./lib/SelectVault.svelte";
  import NewVault from "./lib/NewVault.svelte";
</script>

<Router>
  <main>
    <Route path="/">
      <SelectVault />
    </Route>
    <Route path="/new-vault">
      <NewVault />
    </Route>
  </main>
</Router>

<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    background-color: var(--dark-black);
  }
</style>
