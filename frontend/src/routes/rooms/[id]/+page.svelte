<script lang="ts">
  import type { Room } from '$lib/types/Room';
  import AddRoom from '$lib/components/Room/AddRoom.svelte';
  import RoomView from '$lib/components/Room/RoomView.svelte';

  let { data }: { data: any } = $props();
  let room = $state(data.room as Room);



</script>
<!--
<div>
  <button class="prev left-1"  >Vorheriger</button>
  <span>{room.room_name} ({room.number})</span>
  <button class="next right-1" >Nächster</button>
</div> -->

<RoomView bind:origRoom={room} />

<style>
    span {

    }

    div {
        margin: 0;
        border: 1px solid var(--border);
        height: 3rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    button {
        background-color: var(--primary);
        color: var(--bg-dark);
        margin: 0.4rem;
        width: 6rem;
        padding: 0.4rem;
        border-radius: 0.5rem;
        cursor: pointer;
        font-size: 0.8rem;
    }

</style>



