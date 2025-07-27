<script lang="ts">
  import { onMount } from 'svelte';
  import { Content, Modal, Trigger } from 'sv-popup';
  import RoomCombobox from '$lib/components/Room/RoomCombobox.svelte';

  let close = false;


  onMount(async () => {

  });


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

</script>

<style>

</style>

<Modal close={close} class="popup">
  <Trigger>
    <button>Neue Buchung</button>
  </Trigger>
  <Content>
    <div class="popup-content">
      <form id="add-booking">
        <div class="form-element">
          <label>Von</label>
          <input name="from" type="date">

          <label>Bis</label>
          <input name="to" type="date">
        </div>

        <div class="form-element">
          <label>Bedrooms</label>
          <RoomCombobox />
        </div>

        <div class="form-element">
          <label>Anzahl der Volljährigen</label>
          <input name="anzahl_old" type="number">
        </div>

        <div class="form-element">
          <label>Anzahl der Minderjährigen</label>
          <input name="anzahl_young" type="number">
        </div>

        <div class="form-element">
          <label>Already Checked in</label>
          <input name="checked_in" type="checkbox">
        </div>


        <div class="form-element">
          <label>Frühstück</label>
          <input name="breakfast" type="checkbox">
        </div>

        <div class="form-element">
          <button on:click={saveBooking}>Speichern</button>
        </div>
      </form>
    </div>
  </Content>
</Modal>
