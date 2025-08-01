<script lang="ts">
	import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte';
  import {NotificationDisplay } from '@beyonk/svelte-notifications';

  let themes = {
    danger: 'hsl(9 21% 41%)',
    success: 'hsl(147 19% 36%)',
    warning: '#f0ad4e',
    info: '#5bc0de',
    default: '#aaaaaa'
  }



	let { children } = $props();
</script>
<Navbar />
<NotificationDisplay {themes} />

<main>{@render children()}</main>

<!-- <Footer /> -->
<style>
  main {
      margin: 1rem;
  }
</style>