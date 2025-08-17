<script lang="ts">
  let errorText: string | null = $state(null);
  let data: Credentials = $state({
    password: '',
    username: ''
  });
  let isLoading: boolean = $state(false);

  import { goto } from '$app/navigation';

  type Credentials = {
    password: string;
    username: string;
  }

  async function onsubmit(event: Event) {
    isLoading = true;
    errorText = null;
    event.preventDefault();

    let wait = new Promise(() => setTimeout(() => {
    }, 999999));

    await wait;
    try {
      let resp = await fetch('/api/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
      });
      if (resp.status === 200) {
        errorText = null;
        goto('/dashboard');
      } else if (resp.status === 401) { // unauthorized
        errorText = 'Falsches Passwort oder Benutzername';
      } else {
        errorText = 'Es ist ein Fehler aufgetreten';
      }
    } catch (error) {
      errorText = String(error);
    }
    isLoading = false;
  }
</script>

<div class="centered-container">
  <form action="/api/login" method="post" {onsubmit}>
    <h2 class="text-center">Login</h2>
    <div>
      <label for="username">Username</label>
      <input type="text" name="username" id="username" bind:value={data.username} required />
    </div>

    <div>
      <label for="password">Passwort</label>
      <input type="password" name="password" id="password" bind:value={data.password} />
    </div>

    <div class="error">
      {errorText}
    </div>

    <button class="default" type="submit" aria-label="Login" disabled={isLoading}>
      <div style="visibility: {isLoading ? 'visible' : 'hidden'}" class="spinner" aria-hidden="true"></div>
      <span style="width: var(--s); width: var(--s); padding-left: var(--xss)" class="h-fir w-fit">Login</span>
    </button>
  </form>
</div>

<style>
    .error {
        text-align: left;
        min-height: 1.5rem;
        height: fit-content;
        color: var(--danger);
        font-weight: 600;
    }

    form {
        position: absolute;
        display: flex;
        flex-direction: column;
        align-content: center;
        padding: var(--s) var(--l);
        padding-top: var(--xsss);
        border-radius: var(--border-rad);
        border: 1px solid var(--border);
        background-color: var(--bg);
        width: 28rem;
    }

    label {
        color: var(--text-muted);
        font-size: var(--s);
    }

    h2 {
        font-weight: 600;
    }

    form div {
        display: flex;
        flex-direction: column;
        margin-bottom: var(--xss);
    }

    .centered-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }

    button.default {
        font-size: var(--s);
        display: flex;
        flex-direction: row;
    }

    /*  SPINNER  */
    .spinner {
        margin: 0 ;
        border: var(--xsss) solid var(--bg-dark);
        border-radius: 50%;
        border-top: var(--xsss) solid var(--border-muted);
        width: var(--m);
        height: var(--m);
        animation: spin 1s linear infinite;
        align-self: center;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>