<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { navigate } from "svelte-navigator";

  export let params;

  let url;
  let username;
  let password;

  let showPassword = false;

  let entries = [];

  let error = "";
  async function addVaultEntry() {
    if (!url) {
      error = "No url specified";
      return;
    }
    if (!username) {
      error = "No username specified";
      return;
    }
    if (!password) {
      error = "No password specified";
      return;
    }

    await invoke("add_entry", {
      url: url,
      username: username,
      password: password,
    });
    entries = [...entries, {url: url, username: username, password: password}];
  }

  function copyPassword(i) {
    navigator.clipboard.writeText(entries[i].password);
  }

  onMount(async () => {
    entries = await invoke("get_active_vault_entries");
    console.log(entries);
  });
</script>

<main class="container">
  <div class="wrapper">
    <div class="list-header">
      <h1 id="vault-entry-header">Vault Entries</h1>
    </div>
    <div class="list">
      {#each entries as entry, i}
        <div class="entry-wrapper" on:click={() => copyPassword(i)} on:keydown>
          <span>Url: {entry.url}</span>
          <span>Username: {entry.username}</span>
          {#if showPassword}
            <span>Password: {entry.password}</span>
          {/if}
        </div>
      {/each}
    </div>
  </div>
  <div class="new">
    <h1>{params.vault}</h1>

    <p>Add A New Entry</p>

    <div class="button-wrapper">
      <button id="lock" on:click={() => navigate(-1)}>Lock</button>
      <button id="lock" on:click={() => showPassword = !showPassword}>Show Passwords</button>
    </div>
    <div class="row">
      <input type="text" id="url" placeholder="Entry URL:" bind:value={url} />
      <input
        type="text"
        id="username"
        placeholder="Username:"
        bind:value={username}
      />
      <input
        type="password"
        id="password"
        placeholder="Password:"
        bind:value={password}
      />
      <button on:click={addVaultEntry}>Add Entry</button>
    </div>
    <p>{error}</p>
  </div>
</main>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
  }

  #vault-entry-header {
    text-align: left;
    margin-left: 20px;
  }

  .entry-wrapper {
    margin: 20px;
    display: flex;
    flex-direction: column;
  }

  .entry-wrapper  > span {
    text-align: left;
  }

  .entry-wrapper:hover {
    background-color: #666666;
    cursor: pointer;
  }

  .container {
    display: grid;
    grid-template-columns: 0.6fr 1.4fr;
    grid-template-rows: 1fr;
    gap: 0px 0px;
    grid-template-areas: "list new";
  }

  .list {
    grid-area: list;
    overflow-y: scroll;
    overflow-x: hidden;
    height: 350px;
    width: 300px;
  }

  .new {
    grid-area: new;
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

  .button-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    margin: 8px 0 0 8px;
  }
</style>
