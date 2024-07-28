pub mod delete_user;
pub mod edit_user;
pub mod users_list;
pub mod view_user;

use crate::datetime::validate_date_time;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct FullUserProfile {
    pub uuid: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
    pub language: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct EditableUser {
    #[validate(length(min = 6, max = 12))]
    pub username: String,

    #[validate(email)]
    pub primary_email: String,

    #[validate(length(min = 1, max = 16))]
    pub first_name: String,

    #[validate(length(min = 1, max = 16))]
    pub last_name: String,

    #[validate(custom = "validate_date_time")]
    pub birthday: NaiveDate,

    #[validate(url)]
    pub profile_image: Option<String>,

    #[validate(length(min = 1, max = 2))]
    pub language: String,
}
