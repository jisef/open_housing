<script lang="ts">
  import AddBooking from '$lib/components/booking/AddBooking.svelte';
  import Bookings from '$lib/components/booking/Bookings.svelte';
  let close: boolean = $state(false);

  $effect(() => {
    console.log('effect')
  })

</script>

<main id="main">
  <AddBooking bind:close={close} />
  <Bookings text="Buchungen" arrival={null} />
</main>