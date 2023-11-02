use crate::error::RouterError;
use crate::models::QuranMushaf;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single mushaf
pub async fn mushaf_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranMushaf>, RouterError> {
    use crate::schema::mushafs::dsl::{mushafs, uuid as mushaf_uuid};

    let requested_mushaf_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the single mushaf from the database
        let quran_mushafs: QuranMushaf = mushafs
            .filter(mushaf_uuid.eq(requested_mushaf_uuid))
            .get_result(&mut conn)?;

        Ok(web::Json(quran_mushafs))
    })
    .await
    .unwrap()
}
