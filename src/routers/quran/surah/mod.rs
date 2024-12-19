pub mod surah_add;
pub mod surah_delete;
pub mod surah_edit;
pub mod surah_list;
pub mod surah_view;

use std::hash::Hash;

use crate::{
    filter::{Filters, Order},
    models::{QuranAyah, QuranMushaf},
};
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

// This is for Surah router
// which is faster than SimpleAyah in sorting
#[derive(Eq, Serialize, Clone, Debug)]
pub struct SimpleAyahSurah {
    pub number: u32,
    pub uuid: Uuid,
    pub sajdah: Option<String>,
}

// WARNING: Only hashing 'number' ?
// This can lead to collisions in hashmap if the number is not unique
impl Hash for SimpleAyahSurah {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl Ord for SimpleAyahSurah {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialEq for SimpleAyahSurah {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd for SimpleAyahSurah {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Serialize, Clone, Debug, Deserialize)]
pub struct AyahBismillah {
    pub is_ayah: bool,
    pub text: Option<String>,
}

impl AyahBismillah {
    pub fn from_ayah_fields(is_bismillah: bool, bismillah_text: Option<String>) -> Option<Self> {
        match (is_bismillah, bismillah_text) {
            (true, None) => Some(Self {
                is_ayah: true,
                text: None,
            }),
            (false, Some(text)) => Some(Self {
                is_ayah: false,
                text: Some(text),
            }),
            (false, None) => None,
            (_, _) => None,
        }
    }
}

pub fn calculate_break(
    input: Vec<(QuranAyah, String, Option<i32>, Option<String>)>,
) -> Vec<(SimpleAyah, AyahWord)> {
    // WARNINIG: This only works for line type of word breakers
    let mut result: Vec<(SimpleAyah, AyahWord)> = vec![];
    let mut word_line_count = 1;
    let mut ayah_page_count = 1;
    let mut ayah_juz_count = 1;
    let mut ayah_hizb_count = 1;
    for (ayah, word, maybe_word_break_id, maybe_ayah_break_type) in input {
        if let Some(ayah_break_type_s) = maybe_ayah_break_type.clone() {
            match ayah_break_type_s.as_str() {
                "juz" => {
                    ayah_juz_count += 1;
                }
                "hizb" => {
                    ayah_hizb_count += 1;
                }
                "page" => {
                    ayah_page_count += 1;
                    word_line_count = 0;
                }

                _ => {}
            }
        }

        if let Some(_) = maybe_word_break_id {
            result.push((
                SimpleAyah {
                    number: ayah.ayah_number as u32,
                    uuid: ayah.uuid,
                    sajdah: ayah.sajdah,
                    bismillah: AyahBismillah::from_ayah_fields(
                        ayah.is_bismillah,
                        ayah.bismillah_text.clone(),
                    ),
                    hizb: if maybe_ayah_break_type.is_some() {
                        Some(ayah_hizb_count)
                    } else {
                        None
                    },
                    juz: if maybe_ayah_break_type.is_some() {
                        Some(ayah_juz_count)
                    } else {
                        None
                    },

                    page: if maybe_ayah_break_type.is_some() {
                        Some(ayah_page_count)
                    } else {
                        None
                    },
                },
                AyahWord {
                    line: Some(word_line_count),
                    word,
                },
            ));

            word_line_count += 1;
        } else {
            result.push((
                SimpleAyah {
                    number: ayah.ayah_number as u32,
                    uuid: ayah.uuid,
                    sajdah: ayah.sajdah,
                    bismillah: AyahBismillah::from_ayah_fields(
                        ayah.is_bismillah,
                        ayah.bismillah_text.clone(),
                    ),
                    hizb: if maybe_ayah_break_type.is_some() {
                        Some(ayah_hizb_count)
                    } else {
                        None
                    },
                    juz: if maybe_ayah_break_type.is_some() {
                        Some(ayah_juz_count)
                    } else {
                        None
                    },

                    page: if maybe_ayah_break_type.is_some() {
                        Some(ayah_page_count)
                    } else {
                        None
                    },
                },
                AyahWord { line: None, word },
            ));
        }
    }

    result
}

#[derive(Serialize, Clone, Debug)]
pub struct AyahBismillahInSurah {
    pub is_first_ayah: bool,
    pub text: String,
}

/// The Ayah type that will return in the response
#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    pub number: u32,
    pub uuid: Uuid,
    pub sajdah: Option<String>,
    pub bismillah: Option<AyahBismillah>,
    pub hizb: Option<u16>,
    pub juz: Option<u16>,
    pub page: Option<u32>,
}

/// it contains ayah info and the content
#[derive(Serialize, Clone, Debug)]
pub struct AyahWithText {
    #[serde(flatten)]
    pub ayah: SimpleAyah,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hizb: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub juz: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AyahWord {
    pub word: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u16>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AyahWithWords {
    #[serde(flatten)]
    pub ayah: SimpleAyah,
    pub words: Vec<AyahWord>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AyahTy {
    Text(AyahWithText),
    Words(AyahWithWords),
}

impl AyahTy {
    pub fn format_bismillah_for_surah(&self) -> Option<AyahBismillahInSurah> {
        match self {
            AyahTy::Text(at) => at.ayah.bismillah.clone().map(|bismillah| {
                if bismillah.is_ayah {
                    AyahBismillahInSurah {
                        is_first_ayah: true,
                        text: at.text.clone(),
                    }
                } else {
                    AyahBismillahInSurah {
                        is_first_ayah: false,
                        text: bismillah.text.unwrap_or(String::new()),
                    }
                }
            }),
            AyahTy::Words(at) => at.ayah.bismillah.clone().map(|bismillah| {
                if bismillah.is_ayah {
                    AyahBismillahInSurah {
                        is_first_ayah: true,
                        text: at
                            .words
                            .clone()
                            .into_iter()
                            .map(|w| w.word)
                            .collect::<Vec<String>>()
                            .join(" "),
                    }
                } else {
                    AyahBismillahInSurah {
                        is_first_ayah: false,
                        text: bismillah.text.unwrap_or(String::new()),
                    }
                }
            }),
        }
    }
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
    pub transliteration: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahMushaf {
    pub uuid: Uuid,
    pub short_name: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
}

impl From<QuranMushaf> for SingleSurahMushaf {
    fn from(value: QuranMushaf) -> Self {
        Self {
            uuid: value.uuid,
            short_name: value.short_name,
            name: value.name,
            source: value.source,
        }
    }
}

/// The response type for /surah/{id}
#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahResponse {
    pub uuid: Uuid,
    pub mushaf: SingleSurahMushaf,
    pub names: Vec<SurahName>,
    pub period: Option<String>,
    pub number: i32,
    pub number_of_ayahs: i64,
    pub bismillah: Option<AyahBismillahInSurah>,
}

/// The response type for /surah
#[derive(Serialize, Clone, Debug)]
pub struct SurahListResponse {
    pub uuid: Uuid,
    pub number: i32,
    pub period: Option<String>,
    pub number_of_ayahs: i64,
    pub names: Vec<SurahName>,
}

// TODO: Remove number. number must be generated at api runtime
/// User request body type
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SimpleSurah {
    pub name: String,
    pub name_pronunciation: Option<String>,
    pub name_translation_phrase: Option<String>,
    pub name_transliteration: Option<String>,
    pub period: Option<String>,
    pub number: i32,
    pub mushaf_uuid: Uuid,
}
