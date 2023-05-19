<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    let vaultName: string = "vaultName";
    let vaultPath: string = "vaultPath";
    let masterPassword: string = "masterPassword";

    function next(event: MouseEvent) {
        invoke("create_new_vault", {
            vaultName: vaultName,
            vaultPath: vaultPath,
            masterPassword: masterPassword,
        })
    }
</script>

<div class="new-vault-container">
    <h1>Create New Vault</h1>
    <input type="text" placeholder="Vault Name" bind:value={vaultName}>
    <input type="text" placeholder="Vault Location" bind:value={vaultPath}>
    <!-- <input type="text" placeholder="Recovery Key"> -->
    <input type="password" placeholder="Master Password" bind:value={masterPassword}>
    <button on:click={next}>Next</button>
</div>

<style>
    .new-vault-container {
        display: flex;
        flex-direction: column;
        width: 35%;
        background: red;
    }

    @media only screen and (max-width: 800px) {
        .new-vault-container {
            width: 60%;
        }
    }

    h1 {
       text-align: center;
    }
</style>
