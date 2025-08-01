<script lang="ts">
  import { Content, Trigger, Modal } from 'sv-popup';
  import type { Response } from '$lib/types/Response';
  import {notifier} from '@beyonk/svelte-notifications';

  let close = $state(false);
  let errorText: string | null = $state(null);


  async function saveRoom(event: Event) {
    event.preventDefault();
    let data = getFormData();
    let response: Response = await fetch('/api/rooms', {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(response => response.json()).catch(error => errorText = "Ein Kritischer Fehler ist aufgetreten: " + error.message + '. ');
    if (response.status === 'error') {
      notifier.danger("Fehler aufgetreten" + response.message, 5000);
    } else if (response.status === 'success') {
      notifier.success("Erfolgreich gespeichert", 5000);
    }

    close = true;
  }

  function validate() {
    const form = document.getElementById('add-room') as HTMLFormElement;
    let formData: FormData;
    if (form) {
      formData = new FormData(form);
      let number: number = Number(formData.get('number'));
      let capacity: number = Number(formData.get('capacity'));
      let max_capacity: number = Number(formData.get('max-capacity'));
      let bedrooms: number = Number(formData.get('bedrooms'));

      if (number && capacity) {
        errorText = null;
      } else {
        errorText += 'Nummer und Kapazität müssen angegeben werden. ';
      }

      if (max_capacity) {
        if (max_capacity < capacity) {
          errorText += 'Die Maximale Kapazität darf nicht kleiner als die Kapazität sein. ';
        } else {
          errorText = null;
        }
      }

      if (bedrooms) {
        if (bedrooms < 0) {
          errorText += 'Schlafzimmer: Wert zu klein'
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
      let max_capacity: number = Number(formData.get('max-capacity'));

      let is_apartment: boolean = false;
      is_apartment = formData.get('is_apartment') === 'on';

      let has_kitchen: boolean = false;
      has_kitchen = formData.get('has_kitchen') === 'on';

      let bedrooms: number = Number(formData.get('bedrooms'));

      return {
        number: number,
        name: name,
        capacity: capacity,
        max_capacity: max_capacity,
        is_apartment: is_apartment,
        has_kitchen: has_kitchen,
        bedrooms: bedrooms,
      };
    }
  }


</script>
<Modal close={close} >
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
          <div class="form-element">
            <label>Schlafzimmer</label>
            <input type="number" name="bedrooms" onchange={validate} min="0">
          </div>
        </div>

        <div class="form-group form-element items-center" style="margin-left: var(--xs);">
          <input type="checkbox" name="is_apartment" style="margin-right: 6px;">
          <label for="is_apartment">Apartment</label>
        </div>

        <div class="form-group form-element items-center" style="margin-left: var(--xs);">
          <input type="checkbox" name="has_kitchen" style="margin-right: 6px;" >
          <label for="has_kitchen">Has Kitchen</label>
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




