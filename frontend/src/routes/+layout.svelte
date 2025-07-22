<script lang="ts">
	import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte';
  var SERVER_URL = 'http://localhost:3000';
  import type { Booking } from '$lib';



	let { children } = $props();
</script>
<Navbar />

<main>{@render children()}</main>

<!-- <Footer /> -->
<style>
  main {
      margin: 1rem;
  }
</style>