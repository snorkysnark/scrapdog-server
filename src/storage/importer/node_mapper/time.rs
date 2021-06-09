use crate::parser::UnresolvedTime;
use anyhow::{anyhow, Result};
use chrono::{NaiveDateTime, TimeZone};

pub trait ConvertTime {
    fn local_to_utc(&self, timezone: &impl TimeZone) -> Result<NaiveDateTime>;
}

impl ConvertTime for UnresolvedTime {
    fn local_to_utc(&self, timezone: &impl TimeZone) -> Result<NaiveDateTime> {
        use chrono::LocalResult;

        match timezone.from_local_datetime(&self.0) {
            LocalResult::Single(date) => Ok(date.naive_utc()),
            other => Err(anyhow!("Bad convertion result: {:?}", other)),
        }
    }
}
