<script lang="ts">
  import type { Booking } from '$lib/types/Booking';
  import { goto } from '$app/navigation';

  let { booking }: { booking: Booking} = $props();

  function openDetails() {
    goto(`/bookings/${booking.booking_pk}`);
  }


</script>


<div class="list-element" onclick={openDetails}>
  <div style="position: relative; top: 0px;">
    <span class="float-right">
      <!-- <EditButton component={AddBooking} componentProps={booking}/> -->
    </span>
    <span class="float-right"><!-- checked in button --></span>
  </div>

  <p>Zimmer: {booking.room_name} ({booking.room_number})</p>
  <p>Von {booking.date_start}</p>
  <p>Bis {booking.date_end}</p>
</div>

<style>

div span {
  padding: 0.2rem;
}

.list-element {
  background-color: var(--bg-dark);
}
</style>