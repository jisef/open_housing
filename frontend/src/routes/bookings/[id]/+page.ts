import type { PageLoad } from './$types';
import type { APIResponse } from '$lib/types/APIResponse';
import type { Booking } from '$lib/types/Booking';
export const load: PageLoad = async ({ params, fetch }) => {
  try {
    const res: APIResponse = await fetch(`/api/bookings/${params.id}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json"
      }
    }).then(x => {
      return x.json();
    })
    const booking: Booking = res.data;
    return {
      booking: booking
    }
  } catch (error) {
    console.error(error)
  }
}