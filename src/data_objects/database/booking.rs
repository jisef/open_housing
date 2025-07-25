use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Booking {
    pub booking_pk: i32,
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
    pub with_breakfast: bool,
    pub valid: bool,
    pub room_fk: i32,
    pub num_full_aged_guests: Option<i32>,
    pub num_children: Option<i32>,
    pub checked_in: bool
}

impl<'r> FromRow<'r, PgRow> for Booking {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Booking {
            booking_pk: row.try_get("booking_pk")?,
            date_start: row.try_get("date_start")?,
            date_end: row.try_get("date_end")?,
            with_breakfast: row.try_get("with_breakfast")?,
            valid: row.try_get("valid")?,
            room_fk: row.try_get("room_fk")?,
            num_full_aged_guests: row.try_get("num_full_aged_guests")?,
            num_children: row.try_get("num_children")?,
            checked_in: row.try_get("checked_in")?,
        })
    }
}
