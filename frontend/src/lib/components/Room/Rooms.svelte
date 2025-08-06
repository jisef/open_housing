<script lang="ts">
  import { onMount } from 'svelte';
  import type { IRoom } from '$lib/types/Room';
  import RoomElement from '$lib/components/Room/RoomElement.svelte';


  let rooms: IRoom[] = $state([]);

  let { text, limit = $bindable(5)  }: {text: string, limit : number  } = $props();

  let isOpen = true;

  $effect(() => {
    fetchData();
  });

  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    let data = await fetch('/api/rooms?limit=' + limit, {
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

  async function handleLimitChange() {
    await fetchData();
  }

</script>
<style>

</style>

<div class="page" on:click={() => toggle()}>
  <div ><h2>{text}</h2>
    <div class="limit-input"><label>Maximal </label><input type="number" bind:value={limit} on:change={handleLimitChange}
                                                          min="1">
    </div>
  </div>
  <div class="dropdown-content">
    {#if rooms.length === 0}
      <p class="form-group">Keine Räume vorhanden</p>
    {:else}
      {#each rooms as room}
        <div class="booking">
          <RoomElement room={room} />
        </div>
      {/each}
    {/if}
  </div>
</div>

