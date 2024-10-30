use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{error::RouterError, routers::multip, DbPool};

#[derive(Serialize, Deserialize)]
pub struct LangWithStatus {
    language: String,
    status: &'static str,
}

pub async fn list_phrase(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<LangWithStatus>>, RouterError> {
    use crate::schema::app_phrase_translations::dsl::{
        app_phrase_translations, language as p_t_lang,
    };
    use crate::schema::app_phrases::dsl::{app_phrases, phrase};

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let phrases_count: i64 = app_phrases.count().get_result(&mut conn)?;

        let list: Vec<(String, String)> = app_phrases
            .inner_join(app_phrase_translations)
            .select((p_t_lang, phrase))
            .get_results(&mut conn)?;

        let multi = multip(list, |val| val);

        let mut result: Vec<LangWithStatus> = vec![];

        for (key, value) in multi.into_iter() {
            let stat = if value.len() != phrases_count as usize {
                "incomplete"
            } else {
                "complete"
            };

            result.push(LangWithStatus {
                language: key.clone(),
                status: stat,
            });
        }

        Ok(web::Json(result))
    })
    .await
    .unwrap()
}
