use anyhow::{Result, anyhow};
use chrono::{NaiveDateTime, TimeZone, LocalResult};

#[derive(Debug)]
pub struct UnresolvedTime(NaiveDateTime);

impl UnresolvedTime {
    pub fn from_naive(naive: NaiveDateTime) -> Self {
        UnresolvedTime(naive)
    }

    pub fn resolve(&self, timezone: &impl TimeZone) -> Result<NaiveDateTime> {
        match timezone.from_local_datetime(&self.0) {
            LocalResult::Single(date) => Ok(date.naive_utc()),
            other => Err(anyhow!("Bad convertion result: {:?}", other))
        }
    }
}

