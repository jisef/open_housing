<script lang="js">
  import { onMount } from 'svelte';

  export let text;

  let isOpen = true;
  let serverUrl = "http://127.0.0.1:3000"
  let bookings = [];
  function toggle() {
    isOpen = !isOpen;
  }

  async function fetchData() {
    let data = await fetch('/api/bookings', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(res => res.json());
    if (data.status === "error") {
      return data.error;
    }
    bookings = data.data;
    bookings.forEach(booking => {
      let start = new Date(booking.date_start);
      booking.date_start = start.toLocaleDateString("de-AT");
      let end = new Date(booking.date_end);
      booking.date_end= end.toLocaleDateString("de-AT");
    })
  }
  onMount(async () => {
    fetchData()
  })
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



</style>
<div class="dropdown" on:click={() => toggle()}>
  <div class="dropdown-text"><p>{text}</p></div>
  <div class="dropdown-content">
    {#each bookings as booking}
      <div class="booking">
        <p>Anreise Datum: {booking.date_start}</p>
        <p>Abreise Datum: {booking.date_end}</p>
      </div>
    {/each}
  </div>
</div>

