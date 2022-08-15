use chrono::{DateTime, Utc};

use crate::errors;

pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, errors::DateParseError> {
    // Assuming UTC timezone, because no other timezone is specified
    // FUTURE: Follow up with ambientCG dev about this
    let naive = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
    let utc = chrono::DateTime::from_utc(naive, Utc);
    Ok(utc)
}