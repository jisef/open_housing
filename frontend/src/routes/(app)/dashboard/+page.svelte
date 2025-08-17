<script lang="js">
  import Bookings from '$lib/components/booking/Bookings.svelte';
  import Rooms from '$lib/components/room/Rooms.svelte';
</script>

<style>
</style>

<main>
   <Bookings text="Anreisen Heute" arrival={true}/>

  <Bookings text="Abreisen Heute" arrival={false}/>

  <Rooms text="Zimmer" limit={7} />

</main>