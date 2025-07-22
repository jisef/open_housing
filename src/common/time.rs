use std::str::{FromStr, Split};
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

const URL_ENCODED_COLON: &str = "%3A";

#[derive(Deserialize, Debug, Serialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}
impl FromStr for Time {
    type Err = String; // or a custom error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Time::from(s) {
            Some(time) => Ok(time),
            None => Err(format!("Could not parse to Time: {}", s))
        }
    }
}

impl Time {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Time {
        Time { hours, minutes, seconds }
    }
    pub fn from(time: &str) -> Option<Time> {
        let mut split: Split<&str> = time.split(":");
        let hours: u32 = split.next()?.parse::<u32>().ok()?;
        let minutes: u32 = split.next()?.parse::<u32>().ok()?;
        let seconds: u32 = match split.next() {
            Some(seconds) => {
                seconds.parse::<u32>().ok()?
            }
            None => {0}
        };

        Some(
            Time {
                hours,
                minutes,
                seconds
            }
        )
    }

    pub fn default() -> Time {
        Time {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    pub fn for_fetch(&self) -> String {
        format!(
            "{}{}{}",
            self.hours,
            URL_ENCODED_COLON,
            self.minutes
        )
    }
}

/*impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Time", 3)?;
        state.serialize_field("hours", &self.hours)?;
        state.serialize_field("minutes", &self.minutes)?;
        state.serialize_field("seconds", &self.seconds)?;
        state.end()
    }
}*/
