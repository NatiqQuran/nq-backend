pub mod word_list;
pub mod word_view;
pub mod word_edit;
pub mod word_delete;

use serde::Deserialize;
use uuid::Uuid;

use crate::filter::{Filters, Order};

#[derive(Deserialize)]
pub struct SimpleWord {
    pub ayah_uuid: Uuid,
    pub word: String,
}

#[derive(Deserialize)]
pub struct WordListQuery {
    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for WordListQuery {
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
