import type { PageLoad } from './$types';
import type { Response } from '$lib/types/Response';
import type { Booking } from '$lib/types/Booking';
export const load: PageLoad = async ({ params, fetch }) => {
  try {
    const res: Response = await fetch(`/api/bookings/${params.id}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json"
      }
    }).then(x => {
      console.log(x);
      return x.json();
    })
    const booking: Booking = res.data;
    console.log(booking)
    return {
      booking: booking
    }
  } catch (error) {
    console.error(error)
  }
}