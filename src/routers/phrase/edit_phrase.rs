use std::collections::HashMap;

use actix_web::web;
use diesel::prelude::*;

use crate::{error::RouterError, models::NewPhraseTranslation, DbPool};

pub async fn edit_phrase(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    web::Json(new_phrase): web::Json<HashMap<String, String>>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_phrase_translations::dsl::{
        app_phrase_translations, phrase_id as p_id_t, text,
    };
    use crate::schema::app_phrases::dsl::{app_phrases, id as app_phrase_id, phrase};

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // TODO: WARNING query in loop
        for (key, value) in new_phrase.into_iter() {
            let p_id: i32 = app_phrases
                .filter(phrase.eq(key))
                .select(app_phrase_id)
                .get_result(&mut conn)?;

            diesel::insert_into(app_phrase_translations)
                .values(NewPhraseTranslation {
                    phrase_id: p_id,
                    language: path.clone().as_str(),
                    text: value.clone().as_str(),
                })
                .on_conflict(p_id_t)
                .do_update()
                .set(text.eq(value))
                .execute(&mut conn)?;
        }

        Ok("Edited")
    })
    .await
    .unwrap()
}
