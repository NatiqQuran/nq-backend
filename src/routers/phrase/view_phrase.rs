use actix_web::web;
use diesel::prelude::*;
use std::collections::HashMap;

use crate::error::RouterError;
use crate::DbPool;

pub async fn view_phrase(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<web::Json<HashMap<String, Option<String>>>, RouterError> {
    use crate::schema::app_phrase_translations::dsl::{
        app_phrase_translations, language as phrase_lang, text as phrase_translated,
    };
    use crate::schema::app_phrases::dsl::{app_phrases, phrase};

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // TODO: fix nullable bug
        let phrases: Vec<(String, Option<String>)> = app_phrases
            .left_join(app_phrase_translations)
            .filter(phrase_lang.eq(path.clone()).or(phrase_lang.is_null()))
            .select((phrase, phrase_translated.nullable()))
            .get_results(&mut conn)?;

        Ok(web::Json(HashMap::from_iter(phrases)))
    })
    .await
    .unwrap()
}
