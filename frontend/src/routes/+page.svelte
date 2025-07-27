<script lang="js">
  import Bookings from '$lib/components/booking/Bookings.svelte';
  import Rooms from '$lib/components/Room/Rooms.svelte';
</script>

<style>
</style>

<main>
  <Bookings text="Anreisen Heute" url="/api/bookings/today?arrival=true" />

  <Bookings text="Abreisen Heute" url="/api/bookings/today?arrival=false"/>

  <Rooms text="Zimmer" />
</main>