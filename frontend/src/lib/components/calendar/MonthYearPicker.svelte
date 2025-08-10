<script lang="ts">
  const MONTHS = ['Jänner', 'Februar', 'März', 'April', 'Mai', 'Juni', 'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember'];
  const DAYS = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];


  let { value = $bindable(), disabled = false }: {value: Date, disabled : boolean} = $props();

  let isOpen = $state<boolean>(false);
  let displayYear = $state<number>(new Date().getFullYear());
  let selectedMonth = $derived<number>(value.getMonth());
  let selectedYear = $derived<number>(value.getFullYear());
  let placeholder = $derived(selectedYear + " " + MONTHS[selectedMonth]);


  const currentYear = new Date().getFullYear();
  const yearRange = { min: currentYear - 50, max: currentYear + 10 };


  // Update value when selection changes
  $effect(() => {
    if (selectedMonth !== null && selectedYear !== null) {
      value = new Date(selectedYear, selectedMonth);
    }
  });

  function select(monthIndex: number, year: number): void {
    value = new Date(year, monthIndex);
    if (selectedYear !== null && selectedMonth !== null) {
      isOpen = false;
    }
  }



  function previousYear(): void {
    if (displayYear > yearRange.min) {
      displayYear--;
    }
  }

  function nextYear(): void {
    if (displayYear < yearRange.max) {
      displayYear++;
    }
  }


  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') {
      isOpen = false;
    }
  }

  function handleClickOutside(event: MouseEvent): void {
    const target = event.target as Element;
    if (!target.closest('.month-year-picker')) {
      isOpen = false;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} on:click={handleClickOutside} />

<div class="month-year-picker">
  <button
    type="button"
    class="picker-button"
    class:selected={selectedMonth !== null && selectedYear !== null}
    class:disabled
    onclick={() => !disabled && (isOpen = !isOpen)}
    {disabled}
  >
    <span class="picker-text">{placeholder}</span>
    <svg class="picker-arrow" class:rotated={isOpen} viewBox="0 0 24 24" fill="none" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 9l6 6 6-6"/>
    </svg>
  </button>

  {#if isOpen}
    <div class="picker-dropdown">
      <div class="year-navigation">
        <!-- PREVIOUS -->
        <button
          type="button"
          class="nav-button"
          onclick={previousYear}
          disabled={displayYear <= yearRange.min}
          aria-label="Previous year"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
          </svg>
        </button>

        <!-- current -->
        <span class="year-display">{displayYear}</span>

        <!-- next -->
        <button
          type="button"
          class="nav-button"
          onclick={nextYear}
          disabled={displayYear >= yearRange.max}
          aria-label="Next year"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
          </svg>
        </button>
        <!-- TODAY-->

        <button
          type="button"
          class="year-button"
          onclick={() => {
            select(new Date().getMonth(), new Date().getFullYear());
          }}
        >
          Heute
        </button>
      </div>

      <div class="months-grid">
        {#each MONTHS as month, index}
          <button
            type="button"
            class="month-button"
            class:selected={selectedMonth === index && displayYear === selectedYear}
            class:current-selection={selectedMonth === index && selectedYear === displayYear}
            onclick={() => {
              select(index, displayYear);
            }}
          >
            {month.slice(0, 3)}
          </button>
        {/each}
      </div>


    </div>
  {/if}
</div>

<style>
    .month-year-picker {
        position: relative;
        display: inline-block;
        width: 100%;
        max-width: 280px;
    }

    .picker-button {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 16px;
        border: 1px solid var(--border-muted);
        border-radius: 8px;
        background: white;
        color: var(--text-light);
        font-size: 14px;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .picker-button:hover:not(.disabled) {
        border-color: var(--border);
        background: #f9fafb;
    }

    .picker-button:focus {
        outline: none;
        border-color: var(--border);
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
    }

    .picker-button.selected {
        color: #1f2937;
        border-color: var(--border);
    }

    .picker-button.disabled {
        opacity: 0.5;
        cursor: not-allowed;
        background: #f3f4f6;
    }

    .picker-text {
        flex: 1;
        text-align: left;
    }

    .picker-arrow {
        width: var(--m);
        height: var(--m);
        transition: transform 0.2s ease;
        color: var(--text-light);
    }

    .picker-arrow.rotated {
        transform: rotate(180deg);
    }

    .picker-dropdown {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        margin-top: 4px;
        background: white;
        border: 2px solid var(--border-muted);
        border-radius: 8px;
        box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        z-index: 1000;
        overflow: hidden;
    }

    .year-navigation {
        display: flex;
        align-items: center;
        text-align: center;
        justify-content: space-between;
        padding: 16px;
        border-bottom: 1px solid #e5e7eb;
        background: var(--bg);
    }

    .nav-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        border: 1px solid var(--border-muted);
        border-radius: 6px;
        background: var(--bg-light);
        color: var(--text-muted);
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .nav-button:hover:not(:disabled) {
        background: #f3f4f6;
        border-color: #9ca3af;
    }

    .nav-button:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .nav-button svg {
        width: 16px;
        height: 16px;
    }

    .year-display {
        font-weight: 600;
        font-size: 18px;
        color: #1f2937;
    }

    .months-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: var(--xss);
        padding: var(--s);
        background: var(--bg-light);
    }

    .month-button {
        padding: 10px 6px;
        border: 1px solid var(--border-muted);
        border-radius: 6px;
        background: var(--bg-light);
        color: #374151;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
        text-align: center;
    }

    .month-button:hover {
        background: #f3f4f6;
        border-color: #d1d5db;
    }

    .month-button.selected {
        background: #eff6ff;
        border-color: #3b82f6;
        color: #1d4ed8;
    }

    .month-button.current-selection {
        background: #3b82f6;
        border-color: #3b82f6;
        color: white;
    }


    .year-button {
        padding-right: var(--xss);
        padding-left: var(--xss);
        text-align: center;
        height: 2rem;
        border: 1px solid var(--border-muted);
        border-radius: 6px;
        color: var(--text-muted);
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
        display: inline-block;
        background-color: var(--bg-light);
    }

    .year-button:hover {
        background: #f3f4f6;
        border-color: #9ca3af;
    }

    .year-button.selected {
        background: #3b82f6;
        border-color: #3b82f6;
        color: white;
    }
</style>