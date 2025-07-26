<script lang="ts">
  import { onMount } from 'svelte';
  import BookingElement from '$lib/components/BookingElement.svelte';
  import type { Booking } from '$lib/objects/Booking';

  const defaultLimit = 5;

  let limit = $state(defaultLimit);
  let bookings: Booking[] = $state([]);

  let { text, future } = $props();

  let isOpen = true;

  $effect(() => {
    fetchData();
  });

  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    let data = await fetch('/api/bookings?limit=' + limit + '&future=' + future, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(res => res.json());
    if (data.status === 'error') {
      return data.error;
    }
    bookings = data.data;
    console.log(bookings);
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

  async function handleLimitChange(event: Event) {
    await fetchData();
  }

</script>
<style>
    .dropdown {
        margin: 1rem;
        padding: 1rem;
        border: 1px solid black;
        border-radius: 8px;
        height: auto;
        display: block;
    }

    .limit-input {
        margin: 1px;
        border-bottom: 1px solid hsl(0, 0%, 70%);
        padding: 0.5rem;
    }
</style>

<div class="dropdown page" on:click={() => toggle()}>
  <div class="dropdown-text"><p>{text}</p>
    <div class="limit-input"><label>Maximal</label><input type="number" bind:value={limit} on:change={handleLimitChange}
                                                          min="1">
    </div>
  </div>
  <div class="dropdown-content">
    {#if bookings.length === 0}
      <p class="form-element">Keine Buchungen vorhanden</p>
    {:else}
      {#each bookings as booking}
        <div class="booking">
          <BookingElement start={booking.date_start} end={booking.date_end} room={booking.room_fk.toString()} checked_in={booking.checked_in} />
        </div>
      {/each}
    {/if}

  </div>
</div>

