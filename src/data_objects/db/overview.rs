use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::common::datetime::DateTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct OverviewData {
    pub booking_pk: i32,
    pub room_pk: i32,
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
    pub checked_in: bool,
    pub name: String,
    pub number: i32
}

impl<'r> FromRow<'r, PgRow> for OverviewData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(OverviewData {
            booking_pk: row.try_get("booking_pk")?,
            room_pk: row.try_get("room_pk")?,
            date_start: row.try_get("date_start")?,
            date_end: row.try_get("date_end")?,
            checked_in: row.try_get("checked_in")?,
            name: row.try_get("name")?,
            number: row.try_get("number")?,
        })
    }
}