pub mod mushaf_add;
pub mod mushaf_delete;
pub mod mushaf_edit;
pub mod mushaf_list;
pub mod mushaf_view;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimpleMushaf {
    short_name: String,
    name: String,
    source: String,
    bismillah_text: Option<String>,
}
