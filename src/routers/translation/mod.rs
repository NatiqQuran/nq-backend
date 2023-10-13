pub mod translation_add;
pub mod translation_delete;
pub mod translation_view;
pub mod translation_list;
pub mod translation_edit;
pub mod translation_text_modify;
pub mod translation_text_view;
pub mod translation_text_delete;

use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SimpleTranslation {
    pub translator_account_uuid: Option<Uuid>,
    pub language: String,
    pub release_date: Option<NaiveDate>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTranslationText {
    pub ayah_uuid: Uuid,
    pub text: String,
}
