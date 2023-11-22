use crate::error::RouterError;
use crate::filter::Filter;
use crate::models::QuranMushaf;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

use super::MushafListQuery;

/// Get the lists of mushafs
pub async fn mushaf_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<MushafListQuery>,
) -> Result<web::Json<Vec<QuranMushaf>>, RouterError> {
    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of mushafs from the database
        let quran_mushafs =
            QuranMushaf::filter(Box::from(query))?.load::<QuranMushaf>(&mut conn)?;

        Ok(web::Json(quran_mushafs))
    })
    .await
    .unwrap()
}
