<script lang="ts">
  import { notifier } from '@beyonk/svelte-notifications';
  import type { APIResponse } from '$lib/types/APIResponse';
  import type { Booking } from '$lib/types/Booking';

  const months = ['Jänner', 'Februar', 'März', 'April', 'Mai', 'Juni', 'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember'];
  const days = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];

  let monthDays: string[] = $state([]);
  let calendarMonth: Date = $state(new Date(Date.now()));
  let items: Booking[] = $state([]);
  let currentDay = $derived(calendarMonth.getDay());

  let { room_pk }: { room_pk: number } = $props();

  async function fetchBookings() {
    try {
      let resp: Response = await fetch(`/api/bookings?room_pk=${room_pk}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      });
      if (resp.status === 200) {
        const data: APIResponse = await resp.json() as APIResponse;
        items = data.data;
      } else {
        notifier.danger('Fehler beim Laden der Buchungen' + String(resp));
      }
    } catch (error) {
      notifier.danger(String(error), 5000);
    }
  }

  function getDaysInMonth(): number {
    return new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() + 1, 0).getDate();
  }

  function getFirstDayOfMonth() {
    return new Date(calendarMonth.getFullYear(), calendarMonth.getMonth(), 1).getDay();
  }

  $effect(() => {
    fetchBookings();
  });
  $inspect(items);

  console.log(getFirstDayOfMonth());
</script>

<div class="page">
  <div class="calendar">
    <h2>Calendar</h2>
    <!-- HEADER -->
    <div class="header">
      <div class="content-around items-center flex" style="gap: var(--xss)">
        <button onclick={() => calendarMonth = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() - 1)} aria-label="Next month">
          <svg class="w-6 h-6 text-gray-800 content-center" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"
               width="24" height="24" fill="none" viewBox="0 0 24 24">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="m15 19-7-7 7-7" />
          </svg>
        </button>
        <button class="btn-today" onclick={() => calendarMonth = new Date(Date.now())}>
          Heute
        </button>
        <button onclick={() => calendarMonth = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() + 1)} aria-label="Previous month">
          <svg class="w-6 h-6 text-gray-800" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24"
               height="24"
               fill="none" viewBox="0 0 24 24">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="m9 5 7 7-7 7" />
          </svg>
        </button>
      </div>
      <span>{calendarMonth.getFullYear()}</span> <span>{months[calendarMonth.getMonth()]}</span>
    </div>

    <!-- BODY -->
    <div class="calendar-grid">
      <div class="day-names">
        {#each days as day}
          <span>{day}</span>
        {/each}
      </div>
      <div class="days">
        {#each { length: getDaysInMonth() } as _x, index}
          {#if getFirstDayOfMonth() - 1 <= index }
            <div class="day">
              <span class="day-num">{index - getFirstDayOfMonth() + 2}</span>
              {#each items as item}
                {#if item.date_end && item.date_start}
                  {#if new Date(item.date_start).getDate() === index - getFirstDayOfMonth() + 2}
                    <div class="item start">{item.booking_pk}</div>
                  {:else if new Date(item.date_start).getDate() < index - getFirstDayOfMonth() + 2 && new Date(item.date_end).getDate() > index - getFirstDayOfMonth() + 2}
                    <div class="item in"></div>
                  {:else if new Date(item.date_end).getDate() === index - getFirstDayOfMonth() + 2}
                    <div class="item end"></div>
                  {/if}
                {/if}
              {/each}
            </div>
          {:else}
            <div class="day">Vor</div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>


<style>
    .calendar {
        position: relative;
        display: grid;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: var(--xss);

    }

    .calendar-grid {
        display: grid;
    }

    .day-names {
        display: grid;
        grid-template-columns: repeat(7, 1fr);
        margin-left: var(--xsss);
    }

    .day {
        height: 5rem;
        border: 1px solid var(--border-muted);
        padding: var(--xsss);
        position: relative;
        overflow: visible;
    }

    .days {
        display: grid;
        grid-template-columns: repeat(7, 1fr);
    }

    .day-num {
        font-size: var(--s);
        font-weight: 600;
    }

    .item {
        background-color: hsl(217 42% 51%);
        color: var(--bg-dark);
        margin-bottom: var(--xssss);
        position: absolute;
        text-align: center;
        height: var(--m);
    }

    .item.in {
        left: 0;
        right: 0;
    }

    .item.start {
        border-bottom-left-radius: var(--border-rad-small);
        border-top-left-radius: var(--border-rad-small);
        margin-bottom: var(--xssss);
        right: 0;
        left: 50%;
        margin-left: var(--xsss);
    }

    .item.end {
        border-bottom-right-radius: var(--border-rad-small);
        border-top-right-radius: var(--border-rad-small);
        left: 0;
        right: 50%;
        margin-right: var(--xsss);
    }

    .btn-today {
        padding: var(--xssss);
        padding-inline: var(--xss);
        border: 1px solid var(--border-muted);
        border-radius: var(--border-rad-small);
    }
</style>