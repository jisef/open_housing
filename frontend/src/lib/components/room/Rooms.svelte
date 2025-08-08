<script lang="ts">
  import type { IRoom } from '$lib/types/Room';
  import RoomElement from '$lib/components/room/RoomElement.svelte';

  let rooms: IRoom[] = $state([]);
  let isLoading = $state(false);

  let { text, limit = $bindable(5)  }: {text: string, limit : number  } = $props();

  let isOpen = true;

  $effect(() => {
    fetchData();
  });

  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    isLoading = true;
    let data = await fetch('/api/rooms?limit=' + limit, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      }
    ).then(data => data.json()).catch(err => console.error(err));
    rooms = data.data as IRoom[];
    isLoading = false;
  }

  async function handleLimitChange() {
    await fetchData();
  }

</script>
<style>

</style>

<div class="page" on:click={() => toggle()}>
  <div ><h2>{text}</h2>
    <div class="limit-input"><label style="margin-right: var(--xss)">Maximal </label><input type="number" bind:value={limit} on:change={handleLimitChange}
                                                          min="1">
    </div>
  </div>
  <div class="dropdown-content">
    {#if isLoading}
      <p style="color: var(--text-muted); margin-left: 0.3rem">Lade Zimmer...</p>
    {:else if rooms.length === 0}
      <p style="color: var(--text-muted); margin-left: 0.3rem">Keine Zimmer vorhanden</p>
    {:else}
      {#each rooms as room}
        <div class="booking">
          <RoomElement room={room} />
        </div>
      {/each}
    {/if}
  </div>
</div>

