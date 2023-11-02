pub mod surah_add;
pub mod surah_delete;
pub mod surah_edit;
pub mod surah_list;
pub mod surah_view;

use crate::models::QuranWord;
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

/// Quran text type
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AyahTextType {
    Words(Vec<QuranWord>),
    Text(String),
}

/// The Ayah type that will return in the response
#[derive(PartialOrd, Ord, Eq, Hash, PartialEq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    number: i32,
    uuid: Uuid,
    sajdeh: Option<String>,
}

/// it contains ayah info and the content
#[derive(Serialize, Clone, Debug)]
pub struct Ayah {
    #[serde(flatten)]
    ayah: SimpleAyah,
    content: AyahTextType,
}

/// The final response body
#[derive(Serialize, Clone, Debug)]
pub struct QuranResponseData {
    #[serde(flatten)]
    surah: SingleSurahResponse,
    ayahs: Vec<Ayah>,
}

/// the query for the /surah/{uuid}
/// example /surah/{uuid}?format=word
#[derive(Debug, Clone, Deserialize)]
pub struct GetSurahQuery {
    #[serde(default)]
    format: Format,
}

/// The query needs the mushaf
/// for example /surah?mushaf=hafs
#[derive(Deserialize)]
pub struct SurahListQuery {
    mushaf: String,
}

/// The response type for /surah/{id}
#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahResponse {
    pub mushaf_uuid: Uuid,
    pub mushaf_name: Option<String>,
    pub surah_uuid: Uuid,
    pub surah_name: String,
    pub surah_period: Option<String>,
    pub surah_number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub bismillah_text: Option<String>,
    pub number_of_ayahs: i64,
}

/// The response type for /surah
#[derive(Serialize, Clone, Debug)]
pub struct SurahListResponse {
    pub uuid: Uuid,
    pub name: String,
    pub period: Option<String>,
    pub number: i32,
    pub number_of_ayahs: i64,
}

// TODO: Remove number. number must be generated at api runtime
/// User request body type
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SimpleSurah {
    pub name: String,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub mushaf_uuid: Uuid,
}
