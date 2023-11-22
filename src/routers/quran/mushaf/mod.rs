pub mod mushaf_add;
pub mod mushaf_delete;
pub mod mushaf_edit;
pub mod mushaf_list;
pub mod mushaf_view;

use serde::Deserialize;

use crate::filter::{Order, Filters};

#[derive(Deserialize)]
pub struct SimpleMushaf {
    short_name: String,
    name: String,
    source: String,
    bismillah_text: Option<String>,
}

#[derive(Deserialize)]
pub struct MushafListQuery {
    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for MushafListQuery {
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
