<script lang="ts">
import BookingView from '$lib/components/booking/BookingView.svelte';
import type { Booking } from '$lib/types/Booking';

let { data }: { data: any} = $props();
let booking: Booking[] = $state(data.booking);

</script>

<BookingView bind:origBooking={booking[0]} isNew={false} close={false}/>




<style>

</style>