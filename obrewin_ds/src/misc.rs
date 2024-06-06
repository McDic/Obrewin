use chrono::{DateTime, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};

/// Wrapping struct for `DateTime<Utc>`.
#[derive(Copy, Clone)]
pub struct DateTimeUTC(DateTime<Utc>);

impl From<DateTime<Utc>> for DateTimeUTC {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<DateTimeUTC> for DateTime<Utc> {
    fn from(value: DateTimeUTC) -> Self {
        value.0
    }
}

impl Serialize for DateTimeUTC {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (total_seconds, subsec_nano) = (self.0.timestamp(), self.0.timestamp_subsec_nanos());
        (total_seconds, subsec_nano).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTimeUTC {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (sec, subsec_nano): (i64, u32) = Deserialize::deserialize(deserializer)?;
        if let Some(dt) = chrono::DateTime::from_timestamp(sec, subsec_nano) {
            Ok(dt.into())
        } else {
            Err(DeError::custom("DateTimeUTC deserialization failed"))
        }
    }
}
