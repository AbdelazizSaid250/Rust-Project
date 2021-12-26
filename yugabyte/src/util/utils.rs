use chrono::{NaiveDate, NaiveDateTime};
use lazy_static::*;
use regex::Regex;

pub fn current_timestamp() -> NaiveDateTime {
    chrono::offset::Utc::now().naive_local()
}

pub fn expiration_date(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    sec: i32,
) -> NaiveDateTime {
    NaiveDate::from_ymd(year, month as u32, day as u32)
        .and_hms(hour as u32, min as u32, sec as u32)
}

lazy_static! {
    pub static ref REGEX_FULL_WORD: Regex = Regex::new(r"^[a-zA-Z ._-]*$").unwrap();   // examples: "abdelaziz", "abdelaziz said", "abdelaziz-said", "abdelaziz_said", "abdelaziz.said"
    pub static ref REGEX_WORD: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();   // examples: "abdelaziz"
}
