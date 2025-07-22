<script lang="ts">
  import { onMount } from 'svelte';

  onMount(async () => {
    const form = document.getElementById('add-booking').addEventListener('submit', async (e) => {
      e.preventDefault();
      await saveBooking();
    });
  });

  async function saveBooking() {
    let data = getFormData();
  }

  function getFormData() {
    const form = document.getElementById('add-room');
    const formData = new FormData(form);

    let number: number = Number(formData.get('number'));
    let name: string = formData.get('name') as string;
    let capacity: number = Number(formData.get('capacity'));
    let maxCapacity: number = Number(formData.get('max-capacity'));

    let isApartment : boolean = false;
    if (formData.get('isApartment') === 'on') {
      isApartment = true;
    }

    let hasKitchen: boolean = false;
    if (formData.get('hasKitchen') === 'on') {
      hasKitchen = true;
    }

    let bedrooms: number = Number(formData.get('bedrooms'));

    return {
      number: number,
      name: name,
      capacity: capacity,
      maxCapacity: maxCapacity,
      isApartment: isApartment,
      hasKitchen: hasKitchen,
      bedrooms: bedrooms,
    };
  }
</script>

<style>

</style>

<form id="add-room">
  <label>ROOM: </label><br>
  <input style="border: 1px solid back" type="file" name="photo">

  <div class="form-element">
    <label>Nummer</label>
    <input type="number" name="number">
  </div>

  <div class="form-element">
    <label>Name</label>
    <input type="text" name="name">
  </div>

  <div class="form-element">
    <label>Kapazität</label>
    <input type="number" name="capacity">

    <label>Maximale Kapazität</label>
    <input type="number" name="max-capacity">
  </div>

  <div class="form-element">
    <label>Apartment</label>
    <input type="checkbox" name="isApartment">

    <label>Has Kitchen</label>
    <input type="checkbox" name="hasKitchen">
  </div>


  <div class="form-element">
    <input type="submit">
    <input type="reset">
  </div>
</form>
