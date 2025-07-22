<script lang="ts">
  import { onMount } from 'svelte';

  onMount(async () => {
    const form = document.getElementById('add-booking').addEventListener('submit', async (e) => {
      e.preventDefault();
      await saveBooking();
    });
  });


  async function saveBooking() {
    const data = getFormData();
    //console.log(JSON.stringify(data));
    let response = await fetch('/api/add_booking',
      {
        body: JSON.stringify(data, theReplacer),
        headers: {
          'Content-Type': 'application/json'
        },
        method: 'POST'
      });
    console.log(response);
  }


  function getFormData() {
    const form = document.getElementById('add-booking');
    const formData = new FormData(form as HTMLFormElement);

    let room: number = Number(formData.get('room'));

    let start: Date = Date(formData.get('from'));

    let end: Date = Date(formData.get('to'));

    let breakfast: boolean = false;
    if (formData.get('breakfast') === 'on') {
      breakfast = true;
    }

    let checked_in: boolean = false;
    if (formData.get('checked_in') === 'on') {
      checked_in = true;
    }

    let data: {
      room: number,
      date_start: Date,
      date_end: Date,
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

<div>
  <form id="add-booking" style="border: 1px solid; padding: 1rem;">
    <label>Room</label>
    <input name="room" type="text">
    <div class="form-element">
      <label>Von</label>
      <input name="from" type="date">

      <label>Bis</label>
      <input name="to" type="date">
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
      <input type="submit" class="inline-grid"><br>
      <input type="reset" class="inline-grid">
    </div>
  </form>

</div>