use crate::error::RouterError;
use crate::filter::Filter;
use crate::models::Translation;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

use super::TranslationListQuery;

/// Returns the list of translations
pub async fn translation_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<TranslationListQuery>,
) -> Result<web::Json<Vec<Translation>>, RouterError> {
    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of translations from the database
        let translations_list =
            Translation::filter(Box::from(query))?.load::<Translation>(&mut conn)?;

        Ok(web::Json(translations_list))
    })
    .await
    .unwrap();

    result
}
