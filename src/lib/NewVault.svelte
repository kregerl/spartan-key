<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { navigate } from "svelte-navigator";

    let vaultName: string = "vaultName";
    let vaultPath: string = "vaultPath";
    let recoveryKeyPath: string = "./";
    let masterPassword: string = "masterPassword";

    async function next(event: MouseEvent) {
        await invoke("create_new_vault", {
            vaultName: vaultName,
            vaultPath: vaultPath,
            masterPassword: masterPassword,
        });
        await navigate(`/passwords/${vaultName}`);
    }
</script>

<button id="back" on:click={() => navigate(-1)}>Back</button>
<div class="new-vault-container">
    <h1>Create New Vault</h1>
    <input type="text" id="name" bind:value={vaultName} required />
    <input type="text" id="path" bind:value={vaultPath} required />
    <input type="text" id="recovery" bind:value={recoveryKeyPath} required />
    <input type="password" id="password" bind:value={masterPassword} required />
    <button on:click={next}>Next</button>
</div>

<style>
    .new-vault-container {
        display: flex;
        flex-direction: column;
        width: 35%;
        background-color: var(--medium-black);
    }

    @media only screen and (max-width: 800px) {
        .new-vault-container {
            width: 60%;
        }
    }

    h1 {
       text-align: center;
    }

    #back {
        position: absolute;
        top: 0;
        left: 0;
    }
</style>
