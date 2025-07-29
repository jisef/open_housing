<script lang="ts">
  import { Content, Modal, Trigger } from 'sv-popup';
  import RoomCombobox from '$lib/components/Room/RoomCombobox.svelte';

  let isValid = $state();


  let close = $state(false);

  async function saveBooking(event: Event) {
    event.preventDefault();
    const data = getFormData();
    let response = await fetch('/api/bookings',
      {
        body: JSON.stringify(data, theReplacer),
        headers: {
          'Content-Type': 'application/json'
        },
        method: 'POST'
      }).then(response => response.json()).catch(error => console.error('Error:', error));
    console.log(response);
    close = true;
  }


  function getFormData() {
    const form = document.getElementById('add-booking');
    const formData = new FormData(form as HTMLFormElement);

    let room: number = Number(formData.get('room'));

    let start: string = String(formData.get('from'));

    let end: string = String(formData.get('to'));

    let breakfast: boolean = false;
    if (formData.get('breakfast') === 'on') {
      breakfast = true;
    }

    let checked_in: boolean = false;
    if (formData.get('checked_in') === 'on') {
      checked_in = true;
    }

    if (!room || !start || !end) {
      console.error('Missing data');
      return;
    }

    let data: {
      room: number,
      date_start: string,
      date_end: string,
      adults: number,
      children: number,
      checked_in: boolean,
      breakfast: boolean
    } = {
      'room': room,
      'date_start': start,
      'date_end': end,
      'adults': Number(formData.get('anzahl_old')),
      'children': Number(formData.get('anzahl_young')),
      'checked_in': checked_in,
      'breakfast': breakfast
    };
    console.log(JSON.stringify(data, theReplacer));
    return data;
  }

  function theReplacer(key: string, value: number) {
    return key === 'room' ? +value : value;
  }

  async function checkAvailability() {
    const form = document.getElementById('add-booking') as HTMLFormElement;
    if (form) {
      let url = '/api/rooms/free?';
      let formData: FormData = new FormData(form);
      let from = formData.get('from');
      let to = formData.get('to');
      let room = formData.get('room');

      if (from && to) {
        url += 'from=' + from + '&to=' + to;
        if (room) {
          url += '&room=' + room;
        }
      } else {
        return;
      }

      if (from >= to) {
        isValid = false;
        return;
      } else {
        let response = await fetch(url, {
          method: 'Get',
          headers: {
            'Content-Type': 'application/json'
          }
        }).then(response => response.json()).catch(error => console.error('Error:', error.json()));
        if (response) {
          isValid = response.free as boolean;
        }
      }
    }
  }

</script>

<style>

</style>

<Modal close={close} class="popup">
  <Trigger>
    <button>Neue Buchung</button>
  </Trigger>
  <Content>
    <div class="popup-content">
      <h2>Neue Buchung</h2>
      <form id="add-booking">
        <div class="form-group">
          <div class="form-element">
            <label>Von</label>
            <input name="from" type="date" onchange={checkAvailability}>
          </div>

          <div class="form-element">
            <label>Bis</label>
            <input name="to" type="date" onchange={checkAvailability}>
          </div>
        </div>

        <div class="form-group">
          <div class="form-element">
            <label>Zimmer</label>
            <RoomCombobox onchange={checkAvailability} />
          </div>
        </div>


        <div class="form-group">
          <div class="form-element">
            <label>Anzahl der Volljährigen</label>
            <input name="anzahl_old" type="number">
          </div>

          <div class="form-element">
            <label>Anzahl der Minderjährigen</label>
            <input name="anzahl_young" type="number">
          </div>
        </div>


        <div class="form-group items-center form-element" style="margin-left: var(--xs);">
          <input name="checked_in" type="checkbox" style="margin-right: 8px;" >
          <label for="checked_in" class="checkbox-label">Already Checked in</label>
        </div>


        <div class="form-group items-center form-element" style="margin-left: var(--xs);">
          <input name="breakfast" type="checkbox" style="margin-right: 8px;">
          <label for="breakfast">Frühstück</label>
        </div>
        <div class="form-group">
          <button onclick={saveBooking} disabled={!isValid}>Speichern</button>
        </div>
      </form>
      {#if !isValid}
        <span>Zimmer nicht frei oder Eingabe nicht gültig</span>
      {/if}
    </div>
  </Content>
</Modal>
