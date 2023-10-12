use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::SimpleSurah;

/// Update's single surah
pub async fn surah_edit(
    path: web::Path<String>,
    new_surah: web::Json<SimpleSurah>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs, uuid as mushaf_uuid};
    use crate::schema::quran_surahs::dsl::{
        bismillah_status, bismillah_as_first_ayah, mushaf_id as surah_mushaf_id, name, number, period, quran_surahs,
        uuid as surah_uuid,
    };

    let new_surah = new_surah.into_inner();
    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let new_surah_uuid = Uuid::from_str(&path)?;
        let new_surah_mushaf_uuid = Uuid::from_str(&new_surah.mushaf_uuid)?;

        // Select the mushaf by uuid
        // and get the mushaf id
        let mushaf: i32 = mushafs
            .filter(mushaf_uuid.eq(new_surah_mushaf_uuid))
            .select(mushaf_id)
            .get_result(&mut conn)?;

        diesel::update(quran_surahs.filter(surah_uuid.eq(new_surah_uuid)))
            .set((
                number.eq(new_surah.number),
                surah_mushaf_id.eq(mushaf),
                name.eq(new_surah.name),
                bismillah_status.eq(new_surah.bismillah_status),
                bismillah_as_first_ayah.eq(new_surah.bismillah_as_first_ayah),
                period.eq(new_surah.period),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap();

    result
}
