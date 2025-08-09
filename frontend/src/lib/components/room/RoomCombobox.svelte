<script lang="ts">
  import { notifier } from '@beyonk/svelte-notifications';
  import type { IRoom } from '$lib/types/Room';

  let {
    from = $bindable(),
    to = $bindable(),
    selected = $bindable()
  }: {
    from: Date | null,
    to: Date | null,
    selected: IRoom[]
  } = $props();

  // Initialize selected only if it's truly undefined/null, not if it's an empty array
  if (selected === undefined || selected === null) {
    selected = [];
  }
  // Ensure it's always an array type, but don't reset existing values
  if (!Array.isArray(selected)) {
    selected = [];
  }

  let rooms = $state<IRoom[]>([]);
  let isLoading = $state(false);
  let isOpen = $state(false);
  let searchTerm = $state('');
  let dropdownRef: HTMLDivElement;

  let filteredRooms = $derived(
    rooms.filter(room =>
      room.room_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      room.number.toString().includes(searchTerm) ||
      room.capacity.toString().includes(searchTerm)
    )
  );

  function isRoomSelected(room: IRoom): boolean {
    return selected?.some(selectedRoom => selectedRoom.room_pk === room.room_pk) ?? false;
  }

  function toggleRoom(room: IRoom): void {
    if (!selected) selected = [];

    if (isRoomSelected(room)) {
      selected = selected.filter(selectedRoom => selectedRoom.room_pk !== room.room_pk);
    } else {
      selected = [...selected, room];
    }
  }

  function removeRoom(room: IRoom): void {
    if (!selected) return;
    selected = selected.filter(selectedRoom => selectedRoom.room_pk !== room.room_pk);
  }

  function clearAll(): void {
    selected = [];
  }

  function handleClickOutside(event: MouseEvent): void {
    if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
      isOpen = false;
    }
  }

  $effect(() => {
    fetchRooms();
  });

  // Add/remove click outside listener
  $effect(() => {
    if (isOpen) {
      document.addEventListener('click', handleClickOutside);
      return () => document.removeEventListener('click', handleClickOutside);
    }
  });

  async function fetchRooms(): Promise<void> {
    try {
      isLoading = true;

      //TODO needs reworking
      const params = new URLSearchParams();
      if (from) params.append('from', String(from));
      if (to) params.append('to', String(to));

      const url = `/api/rooms${params.toString() ? '?' + params.toString() : ''}`;

      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch rooms: ${response.status} ${response.statusText}`);
      }

      const data = await response.json();
      rooms = data.data as IRoom[];

    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch rooms';
      notifier.danger(message, 5000);
      console.error('Error fetching rooms:', error);
    } finally {
      isLoading = false;
    }
  }

  function getRoomLabel(room: IRoom): string {
    return `${room.room_name} (${room.number}) - ${room.capacity} guests`;
  }
</script>

<div class="room-combobox" bind:this={dropdownRef}>
  <!-- Selected items display with preserved space -->
  <div class="selected-items">
    {#if selected && selected.length > 0}
      {#each selected as room (room.room_pk)}
        <span class="selected-item">
          {getRoomLabel(room)}
          <button
            type="button"
            class="remove-btn"
            onclick={() => removeRoom(room)}
            disabled={isLoading}
          >
            ×
          </button>
        </span>
      {/each}
    {:else}
      <span class="selected-item">
        Kein Zimmer ausgewählt
      </span>
    {/if}
  </div>

  <!-- Search input, dropdown toggle, and clear button container -->
  <div class="controls-container">
    <div class="input-container">
      <input
        type="text"
        bind:value={searchTerm}
        placeholder={isLoading ? 'Lade Zimmer...' : 'Wähle Zimmer'}
        disabled={isLoading}
        onfocus={() => isOpen = true}
        class="search-input"
      />
      <button
        type="button"
        class="dropdown-btn"
        onclick={() => isOpen = !isOpen}
        disabled={isLoading}
      >
        {isOpen ? '▲' : '▼'}
      </button>
    </div>

    <!-- Clear all button -->
    {#if selected && selected.length > 0}
      <button
        type="button"
        class="clear-all-btn"
        onclick={clearAll}
        disabled={isLoading}
      >
        Clear All
      </button>
    {/if}
  </div>

  <!-- Dropdown list -->
  {#if isOpen}
    <div class="dropdown">
      {#if isLoading}
        <div class="loading">Lade Zimmer...</div>
      {:else if filteredRooms.length === 0}
        <div class="no-results">Keine Zimmer gefunden</div>
      {:else}
        {#each filteredRooms as room (room.room_pk)}
          <button
            type="button"
            class="room-option"
            class:selected={isRoomSelected(room)}
            onclick={() => toggleRoom(room)}
          >
            <div class="room-info">
              <div class="room-name">{room.room_name} (#{room.number})</div>
              <div class="room-details">
                {room.capacity} Gäste• {room.bedrooms} Schlafzimmer{room.bedrooms !== 1 ? 's' : ''}
                {#if room.has_kitchen} • Küche{/if}
                {#if room.is_apartment} • Apartment{/if}
              </div>
            </div>
            <div class="checkbox">
              {isRoomSelected(room) ? '☑' : '☐'}
            </div>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
    .room-combobox {
        position: relative;
        width: 100%;
        font-family: system-ui, -apple-system, sans-serif;
    }

    .selected-items {
        display: flex;
        flex-wrap: wrap;
        gap: 4px;
        min-height: 32px; /* Preserve minimum height */
        align-items: flex-start;
    }

    .selected-item {
        display: inline-flex;
        align-items: center;
        gap: 4px;
        background: #e5e7eb;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 14px;
        height: 24px; /* Fixed height for consistency */
    }

    .remove-btn {
        background: none;
        border: none;
        font-size: 16px;
        cursor: pointer;
        color: #6b7280;
        padding: 0;
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .remove-btn:hover {
        color: #ef4444;
    }

    .remove-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .controls-container {
        display: flex;
        gap: 8px;
        align-items: center;
    }

    .input-container {
        display: flex;
        align-items: center;
        border: 1px solid var(--border);
        border-radius: var(--border-rad);
        background: white;
        flex: 1;
    }

    .search-input {
        flex: 1;
        padding: var(--xs);
        border: none;
        outline: none;
        font-size: 0.87rem;
    }

    .search-input:disabled {
        background: #f9fafb;
        color: #6b7280;
    }

    .dropdown-btn {
        padding: var(--xs);
        border: none;
        background: none;
        cursor: pointer;
        color: #6b7280;
    }

    .dropdown-btn:hover {
        color: #374151;
    }

    .dropdown-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .clear-all-btn {
        background: #ef4444;
        color: white;
        border: none;
        border-radius: 6px;
        padding: 8px 12px;
        cursor: pointer;
        font-size: 14px;
        white-space: nowrap;
        display: flex;
        align-items: center;
    }

    .clear-all-btn:hover {
        background: #dc2626;
    }

    .clear-all-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .dropdown {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        background: white;
        border: 1px solid #d1d5db;
        border-radius: 6px;
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
        max-height: 300px;
        overflow-y: auto;
        z-index: 1000;
        margin-top: 4px;
    }

    .loading, .no-results {
        padding: 16px;
        text-align: center;
        color: #6b7280;
        font-size: 14px;
    }

    .room-option {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        padding: 12px;
        border: none;
        background: none;
        cursor: pointer;
        text-align: left;
        border-bottom: 1px solid #f3f4f6;
    }

    .room-option:hover {
        background: #f9fafb;
    }

    .room-option.selected {
        background: #dbeafe;
    }

    .room-option:last-child {
        border-bottom: none;
    }

    .room-info {
        flex: 1;
    }

    .room-name {
        font-weight: 500;
        color: #111827;
        margin-bottom: 2px;
    }

    .room-details {
        font-size: 12px;
        color: #6b7280;
    }

    .checkbox {
        font-size: 16px;
        color: #3b82f6;
    }
</style>