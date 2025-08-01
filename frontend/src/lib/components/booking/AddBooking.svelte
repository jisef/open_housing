<script lang="ts">
  import { Content, Modal, Trigger } from 'sv-popup';
  import BookingView from '$lib/components/booking/BookingView.svelte';
  import { getDefaultBooking } from '$lib/types/Booking';


  let close = $state(false);

</script>

<style>

</style>

<Modal close={close} class="popup">
  <Trigger>
    <button>Neue Buchung</button>
  </Trigger>
  <Content>
    <BookingView origBooking={getDefaultBooking()} isNew={true} bind:close={close}/>
  </Content>
</Modal>
