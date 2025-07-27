<script lang="ts">
  import { onMount } from 'svelte';
  import type { Room } from '$lib/objects/Room';

  let rooms: Room[] = [];

  onMount( async () => {
    await fetchRooms();
  });

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

<select name="room">
  {#each rooms as room}
    <option value={room.room_pk}>{room.name} ({room.number})</option>
  {/each}
</select>

<style>

</style>