<script lang="ts">
  import { onMount } from 'svelte';
  import type { Room } from '$lib/objects/Room';
  import RoomElement from '$lib/components/Room/RoomElement.svelte';

  const defaultLimit = 5;

  let limit = $state(defaultLimit);
  let rooms: Room[] = $state([]);

  let { text } = $props();

  let isOpen = true;

  $effect(() => {
    fetchData();
  });

  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    let data = await fetch('http://localhost:3000/api/rooms?limit=' + limit, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      }
    ).then(data => data.json()).catch(err => console.error(err));
    rooms = data.data;
  }

  onMount(async () => {
    await fetchData();
  });

  async function handleLimitChange(event: Event) {
    await fetchData();
  }

</script>
<style>

</style>

<div class="dropdown page" on:click={() => toggle()}>
  <div class="dropdown-text"><p>{text}</p>
    <div class="limit-input"><label>Maximal </label><input type="number" bind:value={limit} on:change={handleLimitChange}
                                                          min="1">
    </div>
  </div>
  <div class="dropdown-content">
    {#if rooms.length === 0}
      <p class="form-element">Keine Räume vorhanden</p>
    {:else}
      {#each rooms as room}
        <div class="booking">
          <RoomElement room={room} />
        </div>
      {/each}
    {/if}
  </div>
</div>

