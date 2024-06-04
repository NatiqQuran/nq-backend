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
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs, short_name as mushaf_short_name};
    use crate::schema::translations::dsl::{language, mushaf_id as translation_mushaf_id};

    let pool = pool.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the given language or return the default
        let lang = match query.language {
            Some(ref s) => s.clone(),
            None => "en".to_string(),
        };

        let mushafid: i32 = mushafs
            .filter(mushaf_short_name.eq(query.mushaf.clone()))
            .select(mushaf_id)
            .get_result(&mut conn)?;

        // TODO: FIX
        //let master_account: Vec<i32> = match query.master_account {
        //    Some(uuid) => vec![app_accounts
        //        .filter(account_uuid.eq(uuid))
        //        .select(acc_id)
        //        .get_result(&mut conn)?],
        //    None => vec![],
        //};

        // Get the list of translations from the database
        let translations_list = match Translation::filter(Box::from(query)) {
            Ok(filtred) => filtred,
            Err(err) => return Err(err.log_to_db(pool)),
        }
        .filter(language.eq(lang))
        .filter(translation_mushaf_id.eq(mushafid))
        .select(Translation::as_select())
        .get_results(&mut conn)?;

        Ok(web::Json(translations_list))
    })
    .await
    .unwrap();

    result
}
