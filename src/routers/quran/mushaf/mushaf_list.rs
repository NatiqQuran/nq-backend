use crate::error::RouterError;
use crate::models::QuranMushaf;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

/// Get the lists of mushafs
pub async fn mushaf_list(pool: web::Data<DbPool>) -> Result<web::Json<Vec<QuranMushaf>>, RouterError> {
    use crate::schema::mushafs::dsl::*;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of mushafs from the database
        let quran_mushafs = mushafs.load::<QuranMushaf>(&mut conn)?;

        Ok(web::Json(quran_mushafs))
    })
    .await
    .unwrap()
}
