<script lang="ts">
  import type { Room } from '$lib/objects/Room';

  let { room }: { room: Room} = $props();


</script>

<style>
    div span {
        padding: 0.2rem;
    }

    .list-element {
        background-color: var(--bg-dark);
    }
</style>

<div class="list-element">
  <div style="position: relative; top: 0px;">
    <span class="float-right"></span>
  </div>
  <p>number {room.number}</p>
  <p>name {room.name}</p>
  <p>capacity {room.capacity}</p>
</div>

