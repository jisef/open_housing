<script lang="ts">
  import CheckedInButton from '$lib/components/CheckedInButton.svelte';
  import EditButton from '$lib/components/EditButton.svelte';
  import type { Booking } from '$lib/objects/Booking';
  import AddBooking from '$lib/components/booking/AddBooking.svelte';

  let { booking }: { booking: Booking} = $props();


</script>

<style>

div span {
    padding: 0.2rem;
}
</style>

<div class="list-element">
  <div style="position: relative; top: 0px;">
    <span class="float-right">
      <!-- <EditButton component={AddBooking} componentProps={booking}/> -->
    </span>
    <span class="float-right"><CheckedInButton checkedIn="{booking.checked_in}"/></span>
  </div>
  <p>start {booking.date_start}</p>
  <p>end {booking.date_end}</p>
  <p>room {booking.room_fk}</p>

</div>

