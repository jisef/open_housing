import { getChangedFields } from '$lib/helper/GetDifference';
import { notifier } from '@beyonk/svelte-notifications';
import type { Response } from './Response';
import type { IRoom } from '$lib/types/Room';

export interface Booking {
  booking_pk: number;
  checked_in: boolean;
  checked_out: boolean;
  created_at: string;
  date_end: Date | null;
  date_start: Date | null;
  num_children: number;
  num_full_aged_guests: number;
  rooms: IRoom[] ;
  with_breakfast: boolean;
}

export function getDefaultBooking(): Booking {
  return {
    booking_pk: 0,
    checked_in: false,
    checked_out: false,
    created_at: '',
    date_end: null,
    date_start: null,
    num_children: 0,
    with_breakfast: false,
    num_full_aged_guests: 0,
    rooms: [],
  } as Booking;
}

export async function checkAvailability(booking: Booking): Promise<boolean> {
  let valid = isValid(booking);
  if (!valid) {
    return false;
  }

  //todo: make it work again
  let url = '/api/rooms/free?';
/*
  if (booking.date_start && booking.date_end) {
    url += 'from=' + booking.date_start + '&to=' + booking.date_end;
    if (booking.ro) {
      url += '&room=' + booking.room_fk;
    }
  } else {
    return false;
  }
*/
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

export function isValid(booking: Booking): boolean {
  // TODO: make it work again
  let response: boolean = true;
  if (booking.date_start === null || booking.date_end === null) {
    response = false;
  } else if (booking.date_start >= booking.date_end) {
    response = false;
  }  else if (booking.num_full_aged_guests + booking.num_children <= 0) {
    response = false;
  }

  return response;
}

export function isValidMessage(booking: Booking): string | null {
  let response: string | null = null;
  if (isValid(booking)) {
    return null;
  }
  if (booking.date_start === null || booking.date_end === null) {
    response = "Es müssen Start- und Enddatum angegeben werden"
  } else if (booking.date_start >= booking.date_end) {
    response = 'Startdatum muss vor Enddatum liegen';
  }  else if (booking.num_full_aged_guests + booking.num_children <= 0) {
    response = 'Es muss mindestens eine Person angegeben werden';
  }
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


export async function saveBooking(booking: Booking): Promise<boolean> {
  let x = isValid(booking);
  if (!x) {
    return false;
  }
  const data = JSON.stringify(booking);
  try {
    let resp: Response = await fetch('/api/bookings', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: data
    }).then(x => x.json());
    if (resp.status === 'success') {
      notifier.success('Erfolgreich gespeichert', 5000);
      return true;
    }
  } catch (error) {
    notifier.danger(String(error), 5000);
    return false;
  }

  return false;
}

export async function deleteBooking(booking: Booking): Promise<boolean> {
  try {
    let res: Response = await fetch(`http://localhost:3000/api/bookings/${booking.booking_pk}`, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json"
      }
    }).then(x => x.json())

    if (res.status === "success") {
      notifier.success("Erfolgreich gelöscht", 5000);
      return true;
    }
  } catch (error) {
    notifier.danger(String(error), 5000);
  }

  return false;
}