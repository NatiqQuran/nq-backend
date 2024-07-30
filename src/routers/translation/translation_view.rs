use crate::error::RouterError;
use crate::models::Translation;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single translation
pub async fn translation_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Translation>, RouterError> {
    use crate::schema::translations::dsl::{translations, uuid as translation_uuid};

    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the single translation from the database
        let translation: Translation = translations
            .filter(translation_uuid.eq(path))
            .get_result(&mut conn)?;

        Ok(web::Json(translation))
    })
    .await
    .unwrap()
}
