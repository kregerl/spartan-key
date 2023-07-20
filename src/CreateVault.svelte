<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { navigate } from "svelte-navigator";
    import { save } from "@tauri-apps/api/dialog";
    import { appConfigDir } from '@tauri-apps/api/path';

    let vaultName;
    let vaultPath;
    let password;

    let error = "";
    async function create_vault() {
        if (!vaultName) {
            error = "No vault name specified";
            return;
        }
        if (!vaultPath) {
            error = "No vault path specified";
            return;
        }
        if (!password) {
            error = "No master password specified";
            return;
        }

        await invoke("create_new_vault", {
            vaultName: vaultName,
            vaultPath: vaultPath,
            masterPassword: password,
        });

        navigate(`/Login/${vaultName}`);
    }

    async function browse() {
        vaultPath = await save({
            title: "Choose Vault Location",
            defaultPath: await appConfigDir(),
        });
    }
</script>

<main class="container">
    <h1>Spartan Key</h1>

    <div class="row">
        <a href="https://github.com/EdArdolino/spartan-key" target="_blank">
            <img src="/spartankey.png" class="logo vite" alt="Spartan Key" />
        </a>
    </div>

    <p>Create a New Vault</p>

    <div class="row">
        <input
            id="vault-name"
            type="text"
            placeholder="Vault Name:"
            bind:value={vaultName}
        />

        <div class="path-wrapper">
            <input
                id="vault-path"
                type="text"
                placeholder="Vault Path:"
                bind:value={vaultPath}
            />
            <button id="browse" on:click={() => browse()}
                ><img
                    src="Folder.svg"
                    alt="folder"
                    width="32"
                    height="32"
                /></button
            >
        </div>

        <input
            id="password-input"
            type="password"
            placeholder="Password:"
            bind:value={password}
        />
        <button on:click|preventDefault={create_vault}>Create</button>
        <p>{error}</p>
    </div>
</main>

<style>
    #browse {
        padding: 4px;
        border: none;
        margin: 0px;
        width: 40px;
        height: 40px;
    }

    .path-wrapper > input {
        width: 100%;
    }

    .path-wrapper {
        display: flex;
        flex-direction: row;
        justify-content: center;
        width: 100%;
    }

    input {
        width: auto;
    }

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
