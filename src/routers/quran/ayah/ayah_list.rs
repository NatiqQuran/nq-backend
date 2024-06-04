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
    let pool = pool.into_inner();
    web::block(move || {
        let mut conn = pool.get().unwrap();
        
        let filtered_ayahs = match QuranAyah::filter(Box::from(query)) {
            Ok(filtered) => filtered,
            Err(err) => return Err(err.log_to_db(pool)),
        };

        // Get the list of ayahs from the database
        let ayah_list = filtered_ayahs.load::<QuranAyah>(&mut conn)?;

        Ok(web::Json(ayah_list))
    })
    .await
    .unwrap()
}
