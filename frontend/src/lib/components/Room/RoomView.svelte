<script lang="ts">
  import type { Room } from '$lib/types/Room';
  import type { Response } from '$lib/types/Response';
  import { notifier } from '@beyonk/svelte-notifications';
  import DeleteButton from '$lib/components/DeleteButton.svelte';
  import { goto } from "$app/navigation";

  let { origRoom = $bindable() }: { origRoom: Room } = $props();

  let { room, isUpdated }: { room: Room, isUpdated: boolean } = $state({
    room: {...origRoom},
    isUpdated: false
  }); // TODO: needs to be changed

  $inspect(room);
  $inspect(origRoom);


  function checkUpdated() {
    let x = findChangedFields<Room>(origRoom, room).length > 0;
    isUpdated = x;
  }

  async function updateRoom() {
    // get changes
    let x = findChangedFields<Room>(origRoom, room);
    let data: Record<string, any> = {};
    x.forEach(key => {
      data[key] = room[key];
    });
    let json = JSON.stringify(data);

    // patch
    let resp: Response = await fetch(`/api/rooms/${origRoom.room_pk}`, {
      method: 'PATCH',
      body: json,
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(resp => resp.json()).catch(x => notifier.danger('Fehler beim aktualisieren: ' + x.toString()));
    if (resp.status === 'success') {
      notifier.success('Zimmer aktualisiert', 5000);
      origRoom = { ...room };
      isUpdated = false;
      checkUpdated();
    } else {
      notifier.danger('Fehler beim aktualisieren', 5000);
    }
  }

  async function deleteRoom() {
    let resp: Response = await fetch('/api/rooms/' + origRoom.room_pk, {
      method: 'DELETE'
    }).then(x => x.json()).catch(x => console.error('LEGG EIER' + x));
    if (resp.status === "success" && resp.data === true) {
      notifier.success("Zimmer " + room.room_name + " wurde gelöscht!")
      goto('/rooms');
    } else if (resp.status === "error") {
      notifier.danger("Raum kann nicht gelöscht werden da noch Buchungen vorhanden sind!", 5000)
    } else {
      notifier.info("Raum wurde nicht gelöscht")
    }

  }

  function findChangedFields<T>(oldObj: T, newObj: T): (keyof T)[] {
    return (Object.keys(oldObj) as (keyof T)[]).filter(key => oldObj[key] !== newObj[key]);
  }

</script>
<div class="page">
  <h2>Zimmer</h2>
  <div class="form-group">
    <input style="border: 1px solid back" type="file" name="photo" multiple>
  </div>

  <div class="form-group">
    <div class="form-element">
      <label for="number">Nummer</label>
      <input type="number" name="number" min="0" bind:value={room.number} required onchange={checkUpdated}>
    </div>
  </div>

  <div class="form-group">
    <div class="form-element">
      <label for="name">Name</label>
      <input type="text" name="name" bind:value={room.room_name} onchange={checkUpdated}>
    </div>
  </div>

  <div class="form-group">
    <div class="form-element">
      <label for="capacity">Kapazität</label>
      <input type="number" name="capacity" min="1" bind:value={room.capacity} required onchange={checkUpdated}>
    </div>
    <div class="form-element">
      <label for="max-capacity">Maximale Kapazität</label>
      <input type="number" name="max-capacity" bind:value={room.max_capacity} min="1" onchange={checkUpdated}>
    </div>
    <div class="form-element">
      <label for="bedrooms">Maximale Kapazität</label>
      <input type="number" name="bedrooms" min="0" bind:value={room.bedrooms} onchange={checkUpdated}>
    </div>
  </div>

  <div class="form-group form-element items-center" style="margin-left: var(--xs);">
    <input type="checkbox" name="is_apartment" style="margin-right: 6px;" bind:checked={room.is_apartment}
           onchange={checkUpdated}>
    <label for="is_apartment">Apartment</label>
  </div>

  <div class="form-group form-element items-center" style="margin-left: var(--xs);">
    <input type="checkbox" name="has_kitchen" style="margin-right: 6px;" bind:checked={room.has_kitchen}
           onchange={checkUpdated}>
    <label for="has_kitchen">Has Kitchen</label>
  </div>

  <div class="form-group form-element items-center" style="margin-left: var(--xs);">
    <input type="checkbox" name="valid" style="margin-right: 6px;" bind:checked={room.room_valid}
           onchange={checkUpdated}>
    <label for="valid">Gültig</label>
  </div>

  {#if isUpdated}
    <button onclick={updateRoom}>Aktualisieren</button>
  {/if}

<DeleteButton onclick={deleteRoom} />
</div>


<style>

</style>
