use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;

#[derive(Debug, Iden)]
pub enum Booking {
    Table,
    Id,
    Name,
    BookingDate,
    Description,
    Status,
    Duration
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BookingData {
    pub id: i32,
    //pub name: String,
    //pub booking_date: NaiveDate,
    pub description: Option<String>,
    pub status: String,
    //pub duration: i32,
}


impl<'r> FromRow<'r, PgRow> for BookingData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(BookingData {
            id: row.try_get("id")?,
            //name: row.try_get("name")?,
            //booking_date: row.try_get("booking_date")?,
            description: row.try_get("description")?,
            status: row.try_get("status")?,
            //duration: row.try_get("duration")?,
        })
    }
}
impl BookingData {
    pub fn get_create_statement() -> String {
        let table = Table::create()
            .table(Booking::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Booking::Id)
                    .integer()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Booking::Description).text()).col(
            ColumnDef::new(Booking::Status).char_len(32).default("Active")
        ).build(PostgresQueryBuilder);

        println!("Table create: {}", table);
        table
    }

    pub fn default() -> BookingData {
        BookingData::default()
    }
}


