<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { navigate } from "svelte-navigator";

    export let params;
    $: vaultName = params.vault;

    let value;

    async function open_vault() {
        await invoke("open_vault", {name: vaultName, password: value});
        navigate(`/Entry/${vaultName}`)
    }
</script>

<main class="container">
    <h1>Spartan Key</h1>

    <div class="row">
        <a href="https://github.com/EdArdolino/spartan-key" target="_blank">
            <img src="/spartankey.png" class="logo vite" alt="Spartan Key" />
        </a>
    </div>

    <p>Enter Master Password:</p>

    <div class="row">
        <div class="row">
            <input
                id="password-input"
                type="password"
                placeholder="Password:"
                bind:value
            />
            <button on:click={open_vault}>Unlock</button>
        </div>
    </div>
</main>

<style>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
    }

    .row {
        display: flex;
        flex-direction: column;
        justify-content: center;
        margin: 0 25% 0 25%;
    }

    @media (max-width: 1300px) {
        .row {
            margin: 0 15% 0 15%;
        }
    }
</style>
