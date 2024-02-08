pub mod edit_user;
pub mod user;
pub mod users_list;
pub mod delete_user;

use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

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

#[derive(Deserialize)]
pub struct EditableUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub profile_image: String,
    pub language: String,
}

