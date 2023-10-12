use std::str::FromStr;

use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete's the specific surah
pub async fn surah_delete(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_surahs::dsl::{quran_surahs, uuid as surah_uuid};

    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        diesel::delete(quran_surahs.filter(surah_uuid.eq(uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
