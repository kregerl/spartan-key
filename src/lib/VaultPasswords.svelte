<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { navigate } from "svelte-navigator";

    export let params;
    $: console.log("params", params);

    let entries: string[] = [];

    let url: string;
    let username: string;
    let password: string;

    function back() {
        navigate("/");
    }

    async function addEntry() {
        await invoke("add_entry", {
            url: url,
            username: username,
            password: password
        });
        entries.push(url);
        entries = entries;
    }

    onMount(async () => {
        entries = await invoke("get_active_vault_entries");
    });
</script>

<button id="back" on:click={back}>Back</button>

<div class="wrapper">
    <input id="url" type="text" placeholder="url" bind:value={url}/>
    <input id="username" type="text" placeholder="username" bind:value={username} />
    <input id="password" type="text" placeholder="password" bind:value={password} />
    <button id="add" on:click={addEntry}>Add Entry</button>
</div>

<div class="passwords">
    <h1>{params.vault}</h1>
    {#each entries as entry}
        <h2>{entry}</h2>
    {/each}
</div>

<style>
    .passwords,
    .wrapper {
        display: flex;
        flex-direction: column;
    }

    #back {
        position: absolute;
        top: 0;
        left: 0;
    }

    h1, h2 {
        color: white;
    }
</style>
