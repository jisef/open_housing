import { findChangedFields, getChangedFields } from '$lib/helper/GetDifference';
import { notifier } from '@beyonk/svelte-notifications';
import type { Response } from './Response';

export interface Booking {
  booking_pk: number;
  checked_in: boolean;
  created_at: string;
  date_end: Date;
  date_start: Date;
  num_children: number;
  num_full_aged_guests: number;
  room_fk: number;
  valid: boolean;
  with_breakfast: boolean;
  room_name: string,
  room_number: number;
}

export async function checkAvailability(booking: Booking): Promise<boolean | null>  {
  let valid = isValid(booking);
  if (valid !== null) {
    return null;
  }

  let url = '/api/rooms/free?';

  if (booking.date_start && booking.date_end) {
    url += 'from=' + booking.date_start + '&to=' + booking.date_end;
    if (booking.room_fk) {
      url += '&room=' + booking.room_fk;
    }
  } else {
    return false;
  }

  let response = await fetch(url, {
    method: 'Get',
    headers: {
      'Content-Type': 'application/json'
    }
  }).then(response => response.json()).catch(error => notifier.danger('Error:', error.json()));

  if (response) {
    return response.free as boolean;
  }

  return false;
}

export function isValid(booking: Booking): string | null {
  let response: string | null = null;
  if (booking.date_start >= booking.date_end) {
    response = 'Eingabedaten nicht gültig\n';
  }
  if (booking.room_fk === 0) {
    response = "Es muss ein Zimmer ausgewählt werden";
  }
  //TODO maybe add room

  return response;
}

export async function updateBooking(booking: Booking, origBooking: Booking): Promise<boolean> {
  const data = getChangedFields(origBooking, booking);

  try {
    let resp: Response = await fetch(`/api/bookings/${booking.booking_pk}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    }).then(x => x.json());
    if (resp.status === 'success') {
      notifier.success('Erfolgreich gespeichert', 5000);
      return true;
    } else {
      notifier.danger('Fehler beim Speichern' + resp.message, 5000);
    }
  } catch (error) {
    notifier.danger(String(error), 5000);
  }
  return false;
}