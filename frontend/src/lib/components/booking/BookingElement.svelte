<script lang="ts">
  import type { Booking } from '$lib/types/Booking';
  import { goto } from '$app/navigation';
  import { LOCALE } from '$lib/Constants';

  let { booking, arrival }: { booking: Booking, arrival: boolean | null } = $props();

  function openDetails() {
    goto(`/bookings/${booking.booking_pk}`);
  }


</script>

<div class="list-element relative" onclick={openDetails} >
  <div style="position: relative; top: 0px;">
    <span class="float-right">
      <!-- <EditButton component={AddBooking} componentProps={booking}/> -->
    </span>
    <span class="float-right"><!-- checked in button --></span>
  </div>

  <p>
    <span style="color: var(--text-muted)">Zimmer:</span>
    {#each booking.rooms as room}
      <span style="color: var(--text)">{room.room_name}</span><span> ({room.number})</span>

    {/each}
  </p>
  {#if arrival === true}
    <p><span style="color: var(--text-muted)">Bis: </span><span>{new Date(booking.date_end).toLocaleDateString(LOCALE)}</span></p>
  {:else if arrival === false }
    <p><span style="color: var(--text-muted)">Von: </span><span>{new Date(booking.date_start).toLocaleDateString(LOCALE)}</span></p>
  {:else}
    <p><span style="color: var(--text-muted)">Von: </span><span>{new Date(booking.date_start).toLocaleDateString(LOCALE)}</span></p>
    <p><span style="color: var(--text-muted)">Bis: </span><span>{ new Date(booking.date_end).toLocaleDateString(LOCALE)}</span></p>
  {/if}

  <div class="icon-buttons">
    <!-- <CheckedInButton onclick={changeCheckedIn} /> -->
  </div>
</div>

<style>
div span {
  padding-right: 0.2rem;
}

.list-element {
  background-color: var(--bg-dark);
}
</style>