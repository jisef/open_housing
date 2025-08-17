<script lang="ts">
  import AddRoom from "$lib/components/room/AddRoom.svelte"
  import Rooms from '$lib/components/room/Rooms.svelte';
</script>

<main>
  <AddRoom />
  <Rooms text="Zimmer" limit={100}/>
</main>


<style>

</style>


