use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete's the specific Translation
pub async fn translation_delete(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_translations::dsl::{quran_translations, uuid as translation_uuid};

    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::delete(quran_translations.filter(translation_uuid.eq(path))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
