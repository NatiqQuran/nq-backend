pub mod translation_add;
pub mod translation_delete;
pub mod translation_edit;
pub mod translation_list;
pub mod translation_text_delete;
pub mod translation_text_modify;
pub mod translation_text_view;
pub mod translation_view;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::filter::{Filters, Order};

#[derive(Serialize, Deserialize)]
pub struct SimpleTranslation {
    pub translator_account_uuid: Option<Uuid>,
    pub language: String,
    pub release_date: Option<NaiveDate>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranslationStatus {
    Ok,
    NotApproved,
    Incomplete,
}

#[derive(Serialize, Deserialize)]
pub struct TranslationAyah {
    uuid: Uuid,
    text_uuid: Option<Uuid>,
    number: u32,
    surah_number: u32,
    text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ViewableTranslation {
    pub mushaf_uuid: Uuid,
    pub translator_account_uuid: Uuid,
    pub language: String,
    pub release_date: Option<NaiveDate>,
    pub source: Option<String>,
    pub status: TranslationStatus,
    pub ayahs: Vec<TranslationAyah>,
}

#[derive(Serialize, Deserialize)]
pub struct EditableSimpleTranslation {
    pub translator_account_uuid: Option<Uuid>,
    pub language: String,
    pub release_date: Option<NaiveDate>,
    pub source: Option<String>,
    pub approved: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTranslationText {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TranslationListQuery {
    language: Option<String>,
    mushaf: String,
    translator_account: Option<Uuid>,

    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for TranslationListQuery {
    fn sort(&self) -> Option<String> {
        self.sort.clone()
    }

    fn order(&self) -> Option<Order> {
        self.order.clone()
    }

    fn from(&self) -> Option<u64> {
        self.from
    }

    fn to(&self) -> Option<u64> {
        self.to
    }
}
