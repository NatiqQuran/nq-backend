pub mod surah_add;
pub mod surah_delete;
pub mod surah_edit;
pub mod surah_list;
pub mod surah_view;

use crate::filter::{Filters, Order};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The quran text format Each word has its own uuid
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Text,
    Word,
}

impl Default for Format {
    fn default() -> Self {
        Self::Text
    }
}

/// The Ayah type that will return in the response
#[derive(PartialOrd, Ord, Eq, Hash, PartialEq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    number: i32,
    uuid: Uuid,
    sajdah: Option<String>,
}

/// it contains ayah info and the content
#[derive(Serialize, Clone, Debug)]
pub struct AyahWithText {
    #[serde(flatten)]
    ayah: SimpleAyah,
    text: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AyahWithWords {
    #[serde(flatten)]
    ayah: SimpleAyah,
    words: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AyahTy {
    Text(AyahWithText),
    Words(AyahWithWords),
}

/// The final response body
#[derive(Serialize, Clone, Debug)]
pub struct QuranResponseData {
    #[serde(flatten)]
    surah: SingleSurahResponse,
    ayahs: Vec<AyahTy>,
}

/// the query for the /surah/{uuid}
/// example /surah/{uuid}?format=word
#[derive(Debug, Clone, Deserialize)]
pub struct GetSurahQuery {
    #[serde(default)]
    format: Format,

    lang_code: Option<String>,
}

/// The query needs the mushaf
/// for example /surah?mushaf=hafs
#[derive(Clone, Deserialize)]
pub struct SurahListQuery {
    lang_code: Option<String>,
    mushaf: String,
    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for SurahListQuery {
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

#[derive(Serialize, Clone, Debug)]
pub struct SurahName {
    pub arabic: String,
    pub pronunciation: Option<String>,
    pub translation_phrase: Option<String>,
    pub translation: Option<String>,
}

/// The response type for /surah/{id}
#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahResponse {
    pub mushaf_uuid: Uuid,
    pub mushaf_name: Option<String>,
    pub uuid: Uuid,
    pub name: Vec<SurahName>,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub bismillah_text: Option<String>,
    pub number_of_ayahs: i64,
}

/// The response type for /surah
#[derive(Serialize, Clone, Debug)]
pub struct SurahListResponse {
    pub uuid: Uuid,
    pub name: Vec<SurahName>,
    pub period: Option<String>,
    pub number: i32,
    pub number_of_ayahs: i64,
}

// TODO: Remove number. number must be generated at api runtime
/// User request body type
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SimpleSurah {
    pub name: String,
    pub name_pronunciation: Option<String>,
    pub name_translation_phrase: Option<String>,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub mushaf_uuid: Uuid,
}
