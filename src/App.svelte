<!-- <script lang="ts">
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
</style> -->

<script lang="ts">
  import { Router, Route, navigate } from "svelte-navigator";
  import Login from "./Login.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import CreateVault from "./CreateVault.svelte";
  import Vault from "./Vault.svelte";

  onMount(async () => {
    let vaults = await invoke<string[]>("get_vaults");
    if (vaults.length >= 1) {
      navigate(`/Login/${vaults.at(0)}`);
    }
  });
</script>

<Router>
  <Route path="/">
    <CreateVault />
  </Route>

  <Route path="/Login/:vault" let:params>
    <Login {params} />
  </Route>

  <Route path="/Entry/:vault" let:params>
    <Vault {params}/>
  </Route>
</Router>
