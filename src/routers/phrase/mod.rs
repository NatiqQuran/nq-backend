use serde::{Deserialize, Serialize};

pub mod add_phrase;
pub mod phrase_list;
pub mod view_phrase;
pub mod edit_phrase;
pub mod delete_phrase;

#[derive(Serialize, Deserialize)]
pub struct NewReqPhrase {
    phrase: String,
}
