use crate::error::RouterError;
use crate::filter::Filter;
use crate::models::QuranAyah;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

use super::AyahListQuery;

/// Returns the list of ayahs
pub async fn ayah_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<AyahListQuery>,
) -> Result<web::Json<Vec<QuranAyah>>, RouterError> {
    web::block(move || {
        let mut conn = pool.get().unwrap();

        let filtered_ayahs = QuranAyah::filter(Box::from(query))?;

        // Get the list of ayahs from the database
        let ayah_list = filtered_ayahs.load::<QuranAyah>(&mut conn)?;

        Ok(web::Json(ayah_list))
    })
    .await
    .unwrap()
}
