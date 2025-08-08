<script lang="ts">
  import {
    type Booking,
    isValid,
    updateBooking,
    checkAvailability,
    saveBooking,
    isValidMessage, deleteBooking
  } from '$lib/types/Booking';
  import { findChangedFields } from '$lib/helper/GetDifference';
  import RoomCombobox from '$lib/components/room/RoomCombobox.svelte';
  import DeleteButton from '$lib/components/DeleteButton.svelte';
  import { goto } from '$app/navigation';

  let { origBooking = $bindable(), isNew = $bindable(), close = $bindable() }: {
    origBooking: Booking,
    isNew: boolean,
    close: boolean
  } = $props();

  let { booking, isUpdated, errorText, isAvailable }: {
    booking: Booking,
    isUpdated: boolean,
    errorText: string | null,
    isAvailable: boolean,
  } = $state({
    booking: JSON.parse(JSON.stringify(origBooking)) as Booking,
    isUpdated: false,
    errorText: null,
    isAvailable: true
  });

  async function availabilityChanged() {
    changed();
    if (isValid(booking)) {
      isAvailable = await checkAvailability(booking) as boolean;
    }
  }

  function changed() {
    errorText = isValidMessage(booking);
    isUpdated = findChangedFields(origBooking, booking).length > 0;
  }

  async function updateBooking_click() {
    let successful = await updateBooking(booking, origBooking) as boolean;
    if (successful) {
      origBooking = { ...booking };
      isUpdated = false;
    }
  }

  async function saveBooking_Click() {
    let successful = await saveBooking(booking);
    if (successful) {
      origBooking = { ...booking };
      goto('/bookings');
      close = true;
    }
  }

  async function deleteBooking_Click() {
    let success = await deleteBooking(origBooking);
    if (success) {
      goto('/bookings');
    }
  }

  $inspect('booking rooms: ', booking);
  $inspect('orig booking: ', origBooking);
  $inspect('view: ', booking.date_start);
</script>


<div class="page">
  <div class="form-group">
    <div class="form-element">
      <label for="from">Von</label>
      <input name="from" type="date" onchange={availabilityChanged} bind:value={booking.date_start}>
    </div>

    <div class="form-element">
      <label for="to">Bis</label>
      <input name="to" type="date" onchange={availabilityChanged} bind:value={booking.date_end}>
    </div>
  </div>

  <div class="form-group">
    <div class="form-element">
      <label for="room">Zimmer</label>
      <RoomCombobox
        from={booking.date_start}
        to={booking.date_end}
        bind:selected={booking.rooms}
      />

    </div>

    {#if !isAvailable}
      <div class="form-element content-center">
        <p class="flex" style="color: var(--danger);">Zimmer belegt</p>
      </div>
    {/if}
  </div>


  <div class="form-group">
    <div class="form-element">
      <label for="anzahl_old">Anzahl der Volljährigen</label>
      <input name="anzahl_old" type="number" onchange={changed} bind:value={booking.num_full_aged_guests}>
    </div>

    <div class="form-element">
      <label for="anzahl_young">Anzahl der Minderjährigen</label>
      <input name="anzahl_young" type="number" onchange={changed} bind:value={booking.num_children}>
    </div>
  </div>


  <div class="form-group items-center form-element" style="margin-left: var(--xs);">
    <input name="checked_in" type="checkbox" style="margin-right: 8px;" onchange={changed}
           bind:checked={booking.checked_in}>
    <label for="checked_in" class="checkbox-label">Eingecheckt</label>
  </div>

  <div class="form-group items-center form-element" style="margin-left: var(--xs);">
    <input name="checked_out" type="checkbox" style="margin-right: 8px;" onchange={changed}
           bind:checked={booking.checked_out}>
    <label for="checked_out" class="checkbox-label">Ausgecheckt</label>
  </div>


  <div class="form-group items-center form-element" style="margin-left: var(--xs);">
    <input name="breakfast" type="checkbox" style="margin-right: 8px;" onchange={changed}
           bind:checked={booking.with_breakfast}>
    <label for="breakfast">Frühstück</label>
  </div>


  {#if errorText !== null}
    <p style="color: var(--danger);">{errorText}</p>
  {:else }
    {#if isUpdated && !isNew && isAvailable}
      <button onclick={updateBooking_click}>Aktualisieren</button>
    {/if}
    {#if isNew}
      <button onclick={saveBooking_Click} disabled={!isAvailable}>Speichern</button>
    {/if}
  {/if}

  <div class="icon-buttons">
    {#if !isNew}
      <DeleteButton onclick={deleteBooking_Click} />
    {/if}
  </div>

</div>


<style>

</style>