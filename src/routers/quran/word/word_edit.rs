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
    use crate::schema::quran_ayahs::dsl::{id as ayah_id, quran_ayahs, uuid as ayah_uuid};
    use crate::schema::quran_words::dsl::{
        ayah_id as word_ayah_id, quran_words, uuid as word_uuid, word as word_content,
    };

    let new_word = new_word.into_inner();
    let target_word_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let target_ayah: i32 = quran_ayahs
            .filter(ayah_uuid.eq(new_word.ayah_uuid))
            .select(ayah_id)
            .get_result(&mut conn)?;

        diesel::update(quran_words.filter(word_uuid.eq(target_word_uuid)))
            .set((word_ayah_id.eq(target_ayah), word_content.eq(new_word.word)))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}
