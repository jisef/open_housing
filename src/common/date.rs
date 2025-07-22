use error::Error;
use std::error;
use std::str::FromStr;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::common::datetime::DateTime;
use crate::common::time::Time;

#[derive(Deserialize, Debug, Serialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}
impl FromStr for Date {
    type Err = String; // or a custom error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Date::from(s) {
            Some(date) => Ok(date),
            None => Err(format!("Could not parse to Date: {}", s))
        }
    }
}
impl Date {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::from_str(&*s).map_err(serde::de::Error::custom)
    }
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }
    pub fn from(date: &str) -> Option<Date> {
        let mut year = 0 as u16;
        let mut month = 0 as u8;
        let mut day = 0 as u8;
        let mut splits = date.split("-");
        year = splits.next()?.parse::<u16>().ok()?;
        month = splits.next()?.parse::<u8>().ok()?;
        day = splits.next()?.parse::<u8>().ok()?;

        Some(Date {
            year,
            month,
            day
        })
    }

    pub fn default() -> Date {
        Date {
            year: 0,
            month: 0,
            day: 0,
        }
    }

    pub fn for_fetch(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

/*impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Date", 3)?;
        state.serialize_field("year", &self.year)?;
        state.serialize_field("month", &self.month)?;
        state.serialize_field("day", &self.day)?;
        state.end()
    }
}*/
