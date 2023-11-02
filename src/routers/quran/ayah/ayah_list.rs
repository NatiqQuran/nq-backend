use crate::error::RouterError;
use crate::models::QuranAyah;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

/// Returns the list of ayahs
pub async fn ayah_list(pool: web::Data<DbPool>) -> Result<web::Json<Vec<QuranAyah>>, RouterError> {
    use crate::schema::quran_ayahs::dsl::*;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of ayahs from the database
        let ayah_list = quran_ayahs.load::<QuranAyah>(&mut conn)?;

        Ok(web::Json(ayah_list))
    })
    .await
    .unwrap()
}
