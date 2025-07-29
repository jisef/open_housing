<script lang="ts">
  import { Content, Trigger, Modal } from 'sv-popup';

  let close = false;
  let errorText: string | null = $state(null);


  async function saveRoom(event: Event) {
    event.preventDefault();
    let data = getFormData();
    let response = await fetch('/api/rooms', {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(response => response.json()).catch(error => console.error('Error:', error));

    console.log(response);
    close = true;
  }

  function validate() {
    const form = document.getElementById('add-room') as HTMLFormElement;
    let formData: FormData;
    if (form) {
      formData = new FormData(form);
      let number: number = Number(formData.get('number'));
      let name: string = formData.get('name') as string;
      let capacity: number = Number(formData.get('capacity'));
      let maxCapacity: number = Number(formData.get('max-capacity'));

      if (number && capacity) {
        errorText = null;
      } else {
        errorText += 'Nummer und Kapazität müssen angegeben werden. ';
      }

      if (maxCapacity) {
        if (maxCapacity < capacity) {
          errorText += 'Die Maximale Kapazität darf nicht kleiner als die Kapazität sein. ';
        } else {
          errorText = null;
        }
      }
    }
  }

  function getFormData() {
    const form = document.getElementById('add-room') as HTMLFormElement;
    let formData: FormData;
    if (form) {
      formData = new FormData(form);
      let number: number = Number(formData.get('number'));
      let name: string = formData.get('name') as string;
      let capacity: number = Number(formData.get('capacity'));
      let maxCapacity: number = Number(formData.get('max-capacity'));

      let isApartment: boolean = false;
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
        max_capacity: maxCapacity,
        is_apartment: isApartment,
        has_kitchen: hasKitchen,
        bedrooms: bedrooms
      };
    }

  }


</script>
<Modal close={close} class="popup">
  <Trigger>
    <button>Neues Zimmer</button>
  </Trigger>

  <Content>
    <div class="popup-content">
      <h2>Neues Zimmer</h2>
      <form id="add-room">
        <div class="form-group">
          <input style="border: 1px solid back" type="file" name="photo" multiple>
        </div>

        <div class="form-group">
          <div class="form-element">
          <label>Nummer</label>
            <input type="number" name="number" min="0" onchange={validate} required>
          </div>
        </div>

        <div class="form-group">
          <div class="form-element">
          <label>Name</label>
            <input type="text" name="name" onchange={validate}>
          </div>
        </div>

        <div class="form-group">
          <div class="form-element">
            <label>Kapazität</label>
            <input type="number" name="capacity" min="1" onchange={validate} required>
          </div>
          <div class="form-element">
            <label>Maximale Kapazität</label>
            <input type="number" name="max-capacity" onchange={validate} min="1">
          </div>
        </div>

        <div class="form-group form-element items-center" style="margin-left: var(--xs);">
          <input type="checkbox" name="isApartment" style="margin-right: 6px;">
          <label for="isApartment">Apartment</label>
        </div>

        <div class="form-group form-element items-center" style="margin-left: var(--xs);">
          <input type="checkbox" name="hasKitchen" style="margin-right: 6px;" >
          <label for="hasKitchen">Has Kitchen</label>
        </div>


        <div class="form-group">
          <button onclick={saveRoom} disabled={errorText !== null}>Speichern</button>
        </div>
      </form>
      {#if errorText !== null}
        <p style="color: var(--danger)">{errorText.replaceAll("null", "")}</p>
      {/if}
    </div>
  </Content>
</Modal>




