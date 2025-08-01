<script lang="ts">
  import type { Room } from '$lib/types/Room';
  import { goto } from '$app/navigation';

  let { room }: { room: Room} = $props();

  function openDetail() {
    goto(`/rooms/${room.room_pk}`);
  }

</script>

<style>
    div span {
        padding: 0.2rem;
    }
    .list-element {
        background-color: var(--bg-dark);
    }
</style>

<div class="list-element" onclick={openDetail}>
  <div style="position: relative; top: 0px;">
    <span class="float-right"></span>
  </div>
  <p>number {room.number}</p>
  <p>name {room.room_name}</p>
  <p>capacity {room.capacity}</p>
</div>

