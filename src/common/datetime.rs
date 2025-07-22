use std::str::FromStr;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use sqlx::{Database, Postgres};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use crate::common::date::Date;
use crate::common::time::Time;

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct DateTime {
    time: Time,
    date: Date
}
impl FromStr for DateTime {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_time = DateTime::from(String::from(s));
        if date_time.is_some() {
            Ok(date_time.unwrap())
        } else {
            Err(format!("Could not parse to DateTime: {}", s))
        }
    }
}

impl DateTime {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::from_str(&*s).map_err(serde::de::Error::custom)
    }
    /// Return in ISO 8601 without TZ
    pub fn to_string(&self) -> String {
        let date = self.date.for_fetch();
        let time = self.time.for_fetch();
        format!("{}T{}", date,time)
    }

    pub fn from(date_time: String) -> Option<DateTime> {
        let mut split = date_time.split("T");
        let date = Date::from(split.next()?)?;
        let time= Time::from(split.next()?)?;
        Some(
            DateTime {
                date,
                time
            }
        )
    }
    pub fn default() -> DateTime {
        DateTime {
            time: Time::default(),
            date: Date::default()
        }
    }
}



/*impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("DateTime", 2)?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("time", &self.time)?;
        state.end()
    }
}*/