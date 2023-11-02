use crate::error::RouterError;
use crate::models::QuranAyah;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single ayah
pub async fn ayah_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranAyah>, RouterError> {
    use crate::schema::quran_ayahs::dsl::{quran_ayahs, uuid as ayah_uuid};

    let requested_ayah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the single ayah from the database
        let quran_ayah: QuranAyah = quran_ayahs
            .filter(ayah_uuid.eq(requested_ayah_uuid))
            .get_result(&mut conn)?;

        Ok(web::Json(quran_ayah))
    })
    .await
    .unwrap()
}
