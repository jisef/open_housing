export interface Booking {
  booking_pk: number;
  checked_in: boolean;
  created_at: string;
  date_end: string;
  date_start: string;
  num_children: number;
  num_full_aged_guests: number;
  room_fk: number;
  valid: boolean;
  with_breakfast: boolean; 
}