<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import SelectVault from "./lib/SelectVault.svelte";
  import NewVault from "./lib/NewVault.svelte";
  import VaultPasswords from "./lib/VaultPasswords.svelte";
</script>

<Router>
  <main>
    <Route path="/" component={SelectVault} />
    <Route path="/new-vault" component={NewVault} />
    <Route path="/passwords/:vault" let:params >
      <VaultPasswords params={params}/>  
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
