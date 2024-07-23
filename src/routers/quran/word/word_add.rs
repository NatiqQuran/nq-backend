use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::RouterError,
    models::NewQuranWord,
    DbPool,
};

#[derive(Deserialize, Serialize)]
pub struct ReqWord {
    ayah_uuid: Uuid,
    word: String,
}

pub async fn word_add(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<u32>,
    new_word: web::Json<ReqWord>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_ayahs::dsl::{id as ayah_id, quran_ayahs, uuid as ayah_uid};
    use crate::schema::quran_words::dsl::quran_words;

    let user_id = user_id.into_inner();
    let new_word = new_word.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let target_ayah_id: i32 = quran_ayahs
            .filter(ayah_uid.eq(new_word.ayah_uuid))
            .select(ayah_id)
            .get_result(&mut conn)?;

        NewQuranWord {
            ayah_id: target_ayah_id,
            word: new_word.word.as_str(),
            creator_user_id: user_id as i32,
        }
        .insert_into(quran_words)
        .execute(&mut conn)?;

        Ok("added")
    })
    .await
    .unwrap()
}
