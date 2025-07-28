<script lang="ts">
  import { onMount } from 'svelte';
  import BookingElement from '$lib/components/booking/BookingElement.svelte';
  import type { Booking } from '$lib/objects/Booking';

  const defaultLimit = 5;

  let limit = $state(defaultLimit);
  let bookings: Booking[] = $state([]);

  let { text, arrival }: { text: string, arrival: boolean } = $props();

  let isOpen = true;

  $effect(() => {
    fetchData();
  });

  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    let data = await fetch('/api/bookings/today?arrival=' + arrival + '&limit=' + limit, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(res => res.json()).catch(err => console.log(err));
    if (data.status === 'error') {
      return data.error;
    }
    bookings = data.data;
    bookings.forEach(booking => {
      let start = new Date(booking.date_start);
      booking.date_start = start.toLocaleDateString('de-AT');
      let end = new Date(booking.date_end);
      booking.date_end = end.toLocaleDateString('de-AT');
    });
  }

  onMount(async () => {
    await fetchData();
  });

  async function handleLimitChange() {
    await fetchData();
  }

</script>

<div class="page" onclick={() => toggle()}>
  <div><h2 style="color: var(--text)">{text}</h2>
    <div class="limit-input"><label>Maximal</label><input type="number" bind:value={limit} onchange={handleLimitChange}
                                                          min="1">
    </div>
  </div>
  <div class="dropdown-content">
    {#if bookings.length === 0}
      <p style="color: var(--text-muted); margin-left: 0.3rem" >Keine Buchungen vorhanden</p>
    {:else}
      {#each bookings as booking}
        <div class="booking">
          <BookingElement booking={booking} />
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>

</style>