<script lang="ts">
  import { onMount } from 'svelte';
  import BookingElement from '$lib/components/booking/BookingElement.svelte';
  import type { Booking } from '$lib/types/Booking';
  import type { Response } from '$lib/types/Response';
  import { notifier } from '@beyonk/svelte-notifications';

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
    let url = '?limit=' + limit;
    if (arrival === true || arrival === false) {
      url = '/today?limit=' + limit + '&arrival=' + arrival;
    }
    console.log(url);

    try {
      let data: Response = await fetch('http://localhost:3000/api/bookings' + url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      }).then(res => res.json());
      console.log(data);
      if (data.status === 'error') {
        return;
      }
      bookings = data.data as Booking[];
    } catch (error) {
      notifier.danger(error as string, 5000);
    }
  }


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