use crate::error::{RouterError, RouterErrorDetailBuilder};
use crate::filter::Filter;
use crate::models::Translation;
use crate::DbPool;
use actix_web::{web, HttpRequest};
use diesel::prelude::*;

use super::TranslationListQuery;

/// Returns the list of translations
pub async fn translation_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<TranslationListQuery>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Translation>>, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id, uuid as acc_uuid};
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs, short_name as mushaf_short_name};
    use crate::schema::translations::dsl::{
        language as translation_lang, mushaf_id as translation_mushaf_id, translator_account_id,
    };

    let pool = pool.into_inner();

    let error_detail = RouterErrorDetailBuilder::from_http_request(&req).build();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let mushafid: i32 = mushafs
            .filter(mushaf_short_name.eq(query.mushaf.clone()))
            .select(mushaf_id)
            .get_result(&mut conn)?;

        // Get the list of translations from the database
        let mut translations_list = match Translation::filter(Box::from(query.clone())) {
            Ok(filtred) => filtred,
            Err(err) => return Err(err.log_to_db(pool, error_detail)),
        };

        if let Some(lang) = query.language {
            translations_list = translations_list.filter(translation_lang.eq(lang));
        }

        let translations_list = if let Some(translator_uuid) = query.translator_account {
            let account_id: i32 = app_accounts
                .filter(acc_uuid.eq(translator_uuid))
                .select(acc_id)
                .get_result(&mut conn)?;
            translations_list
                .filter(translation_mushaf_id.eq(mushafid))
                .filter(translator_account_id.eq(account_id))
                .select(Translation::as_select())
                .get_results(&mut conn)?
        } else {
            translations_list
                .filter(translation_mushaf_id.eq(mushafid))
                .select(Translation::as_select())
                .get_results(&mut conn)?
        };

        Ok(web::Json(translations_list))
    })
    .await
    .unwrap()
}
