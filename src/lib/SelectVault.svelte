<script lang="ts">
    import { navigate } from "svelte-navigator";

    function newVault() {
        navigate("/new-vault");
    }
</script>

<div class="vault-container">
    <h1 class="high-emphasis">Select Vault</h1>
    <div class="vault">
        <img src="vault.svg" alt="Vault" />
        <span>Vault 1 Name</span>
    </div>
    <div class="vault">
        <img src="vault.svg" alt="Vault" />
        <span>Vault 2 Name</span>
    </div>
    <div class="vault">
        <img src="vault.svg" alt="Vault" />
        <span>Vault 3 Name</span>
    </div>
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
</style>
