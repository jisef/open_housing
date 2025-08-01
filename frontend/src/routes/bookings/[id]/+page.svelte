<script lang="ts">
import BookingView from '$lib/components/booking/BookingView.svelte';
import type { Booking } from '$lib/types/Booking';

let { data }: { data: any } = $props();
let booking = $state(data.booking as Booking);

</script>

<BookingView bind:origBooking={booking}/>




<style>

</style>