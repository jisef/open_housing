<script>
  let activeLink = 'dashboard';
</script>

<nav>
  <div class="navbar">
    <div class="logo">
      <a href="/" on:click={() => activeLink = 'dashboard'}>Open Housing</a>
    </div>
    <ul class="nav-links">
      <li class:active={activeLink === 'dashboard'}>
        <a href="/" on:click={() => activeLink = 'dashboard'}>Dashboard</a>
      </li>
      <li class:active={activeLink === 'bookings'}>
        <a href="/bookings" on:click={() => activeLink = 'bookings'}>Buchungen</a>
      </li>
      <li class:active={activeLink === 'rooms'}>
        <a href="/rooms/" on:click={() => activeLink = 'rooms'}>Zimmer</a>
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