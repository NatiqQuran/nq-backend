pub mod word_delete;
pub mod word_edit;
pub mod word_view;
pub mod word_add;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimpleWord {
    pub word: String,
}
