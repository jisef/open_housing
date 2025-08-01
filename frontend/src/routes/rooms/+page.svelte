<script lang="ts">
  import AddRoom from "$lib/components/Room/AddRoom.svelte"
  import Rooms from '$lib/components/Room/Rooms.svelte';
</script>

<main>
  <AddRoom />
  <Rooms text="Zimmer" limit={100}/>
</main>


<style>

</style>


