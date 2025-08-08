<script lang="ts">
  import type { IRoom } from '$lib/types/Room';
  import { goto } from '$app/navigation';

  let { room }: { room: IRoom} = $props();

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
  <p><span style="color: var(--text-muted)">Nummer: </span> <span>{room.number}</span></p>
  <p><span style="color: var(--text-muted)">Name: </span> {room.room_name}</p>
</div>

