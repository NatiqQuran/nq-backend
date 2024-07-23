pub mod ayah_add;
pub mod ayah_delete;
pub mod ayah_edit;
pub mod ayah_list;
pub mod ayah_view;

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::filter::{Filters, Order};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sajdeh {
    Mostahab,
    Vajib,
}

impl Display for Sajdeh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mostahab => write!(f, "mostahab"),
            Self::Vajib => write!(f, "vajib"),
        }
    }
}

impl Sajdeh {
    pub fn from_option_string(value: Option<String>) -> Option<Self> {
        let Some(value) = value else {
            return None;
        };

        match value.as_str() {
            "vajib" => Some(Self::Vajib),
            "mostahab" => Some(Self::Mostahab),

            _ => None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleWord {
    uuid: Uuid,
    word: String,
}

#[derive(Serialize, Deserialize)]
pub struct AyahWithContent {
    uuid: Uuid,
    mushaf: Uuid,
    surah: Uuid,
    ayah_number: i32,
    sajdeh: Option<Sajdeh>,
    text: String,
    words: Vec<SimpleWord>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleAyah {
    pub ayah_number: i32,
    pub sajdeh: Option<Sajdeh>,
}

#[derive(Deserialize)]
pub struct AyahListQuery {
    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for AyahListQuery {
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
