use std::collections::HashMap;

use actix_web::web;
use diesel::{dsl::exists, prelude::*};

use crate::{error::RouterError, models::NewPhraseTranslation, DbPool};

pub async fn edit_phrase(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    web::Json(new_phrase): web::Json<HashMap<String, String>>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_phrase_translations::dsl::{
        app_phrase_translations, language as t_lang, phrase_id, text,
    };
    use crate::schema::app_phrases::dsl::{app_phrases, id as app_phrase_id, phrase};

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // TODO: WARNING query in loop
        // TODO: Use (on conflict do update)
        for (key, value) in new_phrase.into_iter() {
            let p_id: i32 = app_phrases
                .filter(phrase.eq(key))
                .select(app_phrase_id)
                .get_result(&mut conn)?;

            let exists: bool = diesel::select(exists(
                app_phrase_translations
                    .filter(phrase_id.eq(p_id))
                    .filter(t_lang.eq(path.clone())),
            ))
            .get_result(&mut conn)?;

            if exists {
                diesel::update(app_phrase_translations)
                    .filter(phrase_id.eq(p_id))
                    .set(text.eq(value))
                    .execute(&mut conn)?;
            } else {
                diesel::insert_into(app_phrase_translations)
                    .values(&NewPhraseTranslation {
                        phrase_id: p_id,
                        language: path.clone().as_str(),
                        text: value.clone().as_str(),
                    })
                    .execute(&mut conn)?;
            }
        }

        Ok("Edited")
    })
    .await
    .unwrap()
}
