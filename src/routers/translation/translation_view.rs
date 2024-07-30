use crate::error::RouterError;
use crate::models::Translation;
use crate::{DbPool, ViewableTranslation};
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single translation
pub async fn translation_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<ViewableTranslation>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, id as account_table_id, uuid as account_uuid,
    };
    use crate::schema::mushafs::dsl::{id as mushaf_table_id, mushafs, uuid as mushaf_table_uuid};
    use crate::schema::translations::dsl::{translations, uuid as translation_uuid};

    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the single translation from the database
        let translation: Translation = translations
            .filter(translation_uuid.eq(path))
            .get_result(&mut conn)?;

        let mushaf_uuid: Uuid = mushafs
            .filter(mushaf_table_id.eq(translation.mushaf_id))
            .select(mushaf_table_uuid)
            .get_result(&mut conn)?;

        let translator_account_uuid: Uuid = app_accounts
            .filter(account_table_id.eq(translation.translator_account_id))
            .select(account_uuid)
            .get_result(&mut conn)?;

        Ok(web::Json(ViewableTranslation {
            completed: translation.completed,
            source: translation.source,
            language: translation.language,
            release_date: translation.release_date,
            mushaf_uuid,
            translator_account_uuid,
        }))
    })
    .await
    .unwrap()
}
