<script lang="ts">
  import { goto } from '$app/navigation';
  import { notifier} from '@beyonk/svelte-notifications';

  let activeLink = $state('dashboard');

  async function onclick(e: Event) {
    e.preventDefault();
    let id: string = (e.target as HTMLAnchorElement).id;

    if (id === 'logout') {
      try {
        let resp = await fetch('/api/logout', {method: 'GET'});
        if (resp.status === 200) {
          goto("/login");
        } else {
          notifier.danger('Fehler beim Logout', 5000);
        }
      } catch (error) {
        notifier.danger(String(error), 5000);
      }
    } else {
      activeLink = id;
      goto(`/${id}`);
    }
  }

</script>

<nav>
  <div class="navbar">
    <div class="logo">
      <a href="/" onclick={() => activeLink = 'dashboard'}>Open Housing</a>
    </div>
    <ul class="nav-links">
      <li class:active={activeLink === 'dashboard'}>
        <a id="dashboard" href="/dashboard" {onclick}>Dashboard</a>
      </li>
      <li class:active={activeLink === 'bookings'}>
        <a id="bookings" href="/bookings" {onclick}>Buchungen</a>
      </li>
      <li class:active={activeLink === 'rooms'}>
        <a id="rooms" href="/rooms/" {onclick}>Zimmer</a>
      </li>
      <li>
        <a id="logout" href="/api/logout" {onclick}>Logout</a>
      </li>
    </ul>
  </div>
</nav>

<style>
    nav {
        background-color: #333;
        color: white;
        padding: 1rem;
    }

    .navbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        max-width: 1200px;
        margin: 0 auto;
    }

    .logo a {
        color: white;
        font-size: 1.5rem;
        font-weight: bold;
        text-decoration: none;
    }

    .nav-links {
        display: flex;
        gap: 1rem;
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .nav-links a {
        color: white;
        text-decoration: none;
        padding: 0.5rem 1rem;
        border-radius: 4px;
    }

    .nav-links a:hover {
        background-color: #555;
    }

    .nav-links li.active a {
        background-color: #007acc;
    }
</style>