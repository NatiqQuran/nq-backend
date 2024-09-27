use actix_web::web;
use diesel::prelude::*;
use std::collections::BTreeMap;

use crate::error::RouterError;
use crate::DbPool;

pub async fn view_phrase(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<web::Json<BTreeMap<String, Option<String>>>, RouterError> {
    use crate::schema::app_phrase_translations::dsl::{
        app_phrase_translations, language as phrase_lang, phrase_id as p_t_id,
        text as phrase_translated,
    };
    use crate::schema::app_phrases::dsl::{app_phrases, id as p_id, phrase};

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let phrases: Vec<(String, Option<String>)> = app_phrases
            .left_join(
                app_phrase_translations.on(phrase_lang.eq(path.clone()).and(p_t_id.eq(p_id))),
            )
            .select((phrase, phrase_translated.nullable()))
            .get_results(&mut conn)?;

        Ok(web::Json(BTreeMap::from_iter(phrases)))
    })
    .await
    .unwrap()
}
