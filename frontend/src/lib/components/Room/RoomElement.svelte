<script lang="ts">
  import EditButton from '$lib/components/EditButton.svelte';
  import type { Room } from '$lib/objects/Room';
  import AddRoom from '$lib/components/Room/AddRoom.svelte';

  let { room }: { room: Room} = $props();


</script>

<style>
    div span {
        padding: 0.2rem;
    }
</style>

<div class="list-element">
  <div style="position: relative; top: 0px;">
    <span class="float-right"><EditButton component={AddRoom} componentProps={room}/></span>
  </div>
  <p>number {room.number}</p>
  <p>name {room.name}</p>
  <p>capacity {room.capacity}</p>
</div>

