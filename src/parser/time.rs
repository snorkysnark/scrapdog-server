use anyhow::{Result, anyhow};
use chrono::{NaiveDateTime, TimeZone, DateTime, LocalResult};

pub fn parse<TZ: TimeZone>(timestr: &str, timezone: &TZ) -> Result<DateTime<TZ>> {
    let naive = NaiveDateTime::parse_from_str(timestr, "%Y%m%d%H%M%S")?;
    match timezone.from_local_datetime(&naive) {
        LocalResult::Single(date) => Ok(date),
        other => Err(anyhow!("Bad convertion result: {:?}", other))
    }
}
