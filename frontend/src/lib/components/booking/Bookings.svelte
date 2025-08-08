<script lang="ts">
  import BookingElement from '$lib/components/booking/BookingElement.svelte';
  import type { Booking } from '$lib/types/Booking';
  import type { Response } from '$lib/types/Response';
  import { notifier } from '@beyonk/svelte-notifications';

  const defaultLimit = 5;

  let limit = $state(defaultLimit);
  let bookings: Booking[] = $state([]);
  let isLoading: boolean = $state(false);

  let { text, arrival }: { text: string, arrival: boolean | null } = $props();

  let isOpen = true;

  $effect(() => {
    fetchData();
  });

  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    isLoading = true;
    let url = '?limit=' + limit;
    if (arrival === true || arrival === false) {
      url = '/today?limit=' + limit + '&arrival=' + arrival;
    }
    console.log(url);

    try {
      let data: Response = await fetch('/api/bookings' + url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      }).then(res => res.json());
      console.log(data);
      if (data.status === 'error') {
        isLoading = false;
        return;
      }
      bookings = data.data as Booking[];
      console.log(data.data);
    } catch (error) {
      notifier.danger(error as string, 5000);
    }
    isLoading = false;
  }


  async function handleLimitChange() {
    await fetchData();
  }

</script>

<div class="page" onclick={() => toggle()}>
  <div><h2 style="color: var(--text)">{text}</h2>
    <div class="limit-input"><label style="margin-right: var(--xss)">Maximal</label><input type="number" bind:value={limit} onchange={handleLimitChange}
                                                          min="1">
    </div>
  </div>
  <div class="dropdown-content">
    {#if isLoading}
      <p style="color: var(--text-muted); margin-left: 0.3rem">Lade Buchungen...</p>
    {:else if bookings.length === 0}
      <p style="color: var(--text-muted); margin-left: 0.3rem" >Keine Buchungen vorhanden</p>
    {:else}
      {#each bookings as booking}
        <div class="booking">
          <BookingElement booking={booking} {arrival} />
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>

</style>