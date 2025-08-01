<script lang="ts">
  import { onMount } from 'svelte';
  import type { Room } from '$lib/types/Room';

  type ChangeFunction = (event: Event) => void;

  let {onchange, selected = $bindable() }: {onchange: ChangeFunction, selected: number} = $props();



  let rooms: Room[] = $state([]);

  onMount( async () => {
    await fetchRooms();
  });

  function handleRoomChange(event: Event) {
    if (onchange) {
      onchange(event);
    }
  }

  async function fetchRooms() {
    let data = await fetch('/api/rooms', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(res => res.json()).catch(err => console.log(err));
    rooms = data.data;
  }
</script>

<select name="room" onchange={handleRoomChange} bind:value={selected}>
  {#if rooms.length === 0}
    <option value={0}>Es wurde kein Raum gefunden</option>
    {:else}
    <option value={0}>Keine Auswahl</option>
  {/if}
  {#each rooms as room}
    <option value={room.room_pk}>{room.room_name} ({room.number})</option>
  {/each}
</select>

<style>

</style>