<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { navigate } from "svelte-navigator";

    let dialog: HTMLDialogElement;
    let masterPassword: string;
    let vault: string;

    function newVault() {
        navigate("/new-vault");
    }

    async function getVaults(): Promise<string[]> {
        return await invoke("get_vaults");
    }

    async function submit(e: KeyboardEvent) {
        if (e.key === "Enter") {
            console.log("vault:", vault);
            console.log("masterPassword:", masterPassword);
            await invoke("open_vault", {name: vault, password: masterPassword});
            navigate(`/passwords/${vault}`);
        }
    }

    function openVault() {
        console.log(dialog);
        vault = this.id;
        dialog.showModal();
    }

</script>

<dialog bind:this={dialog}>
    <input type="password" placeholder="Master Password" bind:value={masterPassword} on:keydown={submit} />
</dialog>

<div class="vault-container">
    <h1 class="high-emphasis">Select Vault</h1>
    {#await getVaults()}
        <h1>Loading</h1>
    {:then vaults}
        {#each vaults as vault}
            <div id={vault} class="vault" on:click={openVault} on:keydown>
                <img src="vault.svg" alt="Vault" />
                <span>{vault}</span>
            </div>
        {/each}
    {/await}
    <button id="new-vault" on:click={newVault}>
        <img src="plus.svg" alt="Create New Vault" />
        <span>Create New Vault</span>
    </button>
</div>

<style>
    .vault-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: 35%;
        border-radius: 6px;
        background-color: var(--medium-black);
    }

    @media only screen and (max-width: 800px) {
        .vault-container {
            width: 60%;
        }
    }

    .vault,
    #new-vault {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        height: 64px;
        margin: 8px;
        color: white;
        background-color: var(--light-black);
        cursor: pointer;
    }

    .vault:hover,
    #new-vault:hover {
        background-color: var(--lightest-black);
    }

    #new-vault {
        border: none;
    }

    h1 {
        color: white;
        text-align: center;
    }

    img {
        width: 32px;
    }

    span {
        font-size: 1.6rem;
        font-weight: bold;
        margin: 0 0 0 8px;
    }
    dialog {
        padding: 16px;
        border: 2px solid red;
    }
</style>
