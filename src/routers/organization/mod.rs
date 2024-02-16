pub mod add;
pub mod delete;
pub mod edit;
pub mod list;
pub mod name;
pub mod view;

use crate::datetime::{parse_date_time_with_format, validate_date_time};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Serialize)]
/// Editable Organization which will be recived from client request
pub struct ReqOrganization {
    #[validate(length(min = 6, max = 12))]
    pub username: String,

    #[validate(length(min = 1, max = 16))]
    pub name: String,

    #[validate(url)]
    pub profile_image: Option<String>,

    #[validate(custom = "validate_date_time")]
    #[serde(deserialize_with = "parse_date_time_with_format")]
    pub established_date: NaiveDate,

    #[validate(length(equal = 11))]
    pub national_id: String,
}
