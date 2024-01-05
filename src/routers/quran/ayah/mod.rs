pub mod ayah_add;
pub mod ayah_delete;
pub mod ayah_edit;
pub mod ayah_list;
pub mod ayah_view;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct SimpleAyah {
    pub surah_uuid: String,
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
