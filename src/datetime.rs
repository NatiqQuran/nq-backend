use chrono::{Datelike, NaiveDate, Utc};
use serde::{de, Deserialize, Deserializer};
use validator::ValidationError;

/// parse the date time
pub fn parse_date_time_with_format<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

const OLDEST_YEAR_LIMIT: u32 = 200;

/// This function will validate NaiveDate object
///
/// The date must not pass current time, and not to be too old.
pub fn validate_date_time(date: &NaiveDate) -> Result<(), ValidationError> {
    let current_year = Utc::now().year();
    let input_year = date.year();

    if input_year > current_year {
        return Err(ValidationError::new(
            "The date year is more than current year.",
        ));
    }

    if (current_year - input_year) as u32 > OLDEST_YEAR_LIMIT {
        return Err(ValidationError::new(
            "The date year is older than 200 years.",
        ));
    }

    Ok(())
}
