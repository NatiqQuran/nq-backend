pub mod logout;
pub mod send_code;
pub mod verify;

use chrono::{offset::Utc, DateTime, Duration, NaiveDateTime};

// This constants will make code length
// equal to 6
pub const MIN_RANDOM_CODE: i32 = 100000;
pub const MAX_RANDOM_CODE: i32 = 999999;

/// Get deference between Current time and past_time
pub fn time_deference(past_time: NaiveDateTime) -> Duration {
    let current_date = Utc::now();
    let past_date_time = DateTime::<Utc>::from_utc(past_time, Utc);

    current_date.signed_duration_since(past_date_time)
}
