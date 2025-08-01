<script lang="ts">
  import { type Booking, isValid, updateBooking, checkAvailability  } from '$lib/types/Booking';
  import { notifier } from '@beyonk/svelte-notifications';
  import { findChangedFields } from '$lib/helper/GetDifference';
  import RoomCombobox from '$lib/components/Room/RoomCombobox.svelte';

  let { origBooking = $bindable() }: { origBooking: Booking } = $props();

  let { booking, isUpdated, errorText }: { booking: Booking, isUpdated: boolean, errorText: string | null } = $state({
    booking: { ...origBooking },
    isUpdated: false,
    errorText: null
  });

  async function availabilityChanged(event: Event) {
    changed();
    errorText = isValid(booking);
    let availability = await checkAvailability(booking);
  }

  function changed() {
    if (findChangedFields(origBooking, booking).length > 0) {
      isUpdated = true;
    } else {
      isUpdated = false;
    }
  }

  async function updateBooking_click() {
    let successfull = await updateBooking(booking, origBooking) as boolean;
    if (successfull) {
      origBooking = { ...booking };
      isUpdated = false;
    }
  }
</script>


<div class="page">
  <div class="form-group">
    <div class="form-element">
      <label>Von</label>
      <input name="from" type="date" onchange={availabilityChanged} bind:value={booking.date_start}>
    </div>

    <div class="form-element">
      <label>Bis</label>
      <input name="to" type="date" onchange={availabilityChanged} bind:value={booking.date_end}>
    </div>
  </div>

  <div class="form-group">
    <div class="form-element">
      <label>Zimmer</label>
      <RoomCombobox onchange={availabilityChanged} bind:selected={booking.room_fk} />
    </div>
  </div>


  <div class="form-group">
    <div class="form-element">
      <label>Anzahl der Volljährigen</label>
      <input name="anzahl_old" type="number" onchange={changed} bind:value={booking.num_full_aged_guests}>
    </div>

    <div class="form-element">
      <label>Anzahl der Minderjährigen</label>
      <input name="anzahl_young" type="number" onchange={changed} bind:value={booking.num_children}>
    </div>
  </div>


  <div class="form-group items-center form-element" style="margin-left: var(--xs);">
    <input name="checked_in" type="checkbox" style="margin-right: 8px;" onchange={changed}
           bind:checked={booking.checked_in}>
    <label for="checked_in" class="checkbox-label">Already Checked in</label>
  </div>


  <div class="form-group items-center form-element" style="margin-left: var(--xs);">
    <input name="breakfast" type="checkbox" style="margin-right: 8px;" onchange={changed}
           bind:checked={booking.with_breakfast}>
    <label for="breakfast">Frühstück</label>
  </div>

  {#if errorText}
    {errorText}
  {:else }
    {#if isUpdated}
      <button onclick={updateBooking_click}>Aktualisieren</button>
    {/if}
  {/if}
</div>


<style>

</style>