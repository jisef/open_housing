<script lang="js">
  import { onMount } from 'svelte';
  import Bookings from '$lib/components/Bookings.svelte';
  import BookingElement from '$lib/components/BookingElement.svelte';

  var SERVER_URL = 'http://127.0.0.1:3000';

  async function getBookings() {
    let data = await fetch(SERVER_URL + '/bookings');
  }
</script>

<style>
</style>

<main>
  <Bookings text="Zukünftige Buchungen" future="{true}" />

  <Bookings text="Vergangene Buchungen" future="{false}"/>
</main>