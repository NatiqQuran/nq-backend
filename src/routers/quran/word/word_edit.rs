use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::SimpleWord;

/// Update's single quran_word
pub async fn word_edit(
    path: web::Path<Uuid>,
    new_word: web::Json<SimpleWord>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_words::dsl::{quran_words, uuid as word_uuid, word as word_content};

    let new_word = new_word.into_inner();
    let target_word_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::update(quran_words.filter(word_uuid.eq(target_word_uuid)))
            .set(word_content.eq(new_word.word))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}
