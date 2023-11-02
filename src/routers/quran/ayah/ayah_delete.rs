use crate::error::RouterError;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Delete's a single ayah
pub async fn ayah_delete(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_ayahs::dsl::{quran_ayahs, uuid as ayah_uuid};

    let target_ayah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::delete(quran_ayahs.filter(ayah_uuid.eq(target_ayah_uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
