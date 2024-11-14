use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::SimpleSurah;

/// Update's single surah
pub async fn surah_edit(
    path: web::Path<Uuid>,
    new_surah: web::Json<SimpleSurah>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_mushafs::dsl::{id as mushaf_id, quran_mushafs, uuid as mushaf_uuid};
    use crate::schema::quran_surahs::dsl::{
        bismillah_as_first_ayah, bismillah_status, mushaf_id as surah_mushaf_id, name,
        name_pronunciation, name_translation_phrase, name_transliteration, number, period,
        quran_surahs, uuid as surah_uuid,
    };

    let new_surah = new_surah.into_inner();
    let target_surah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Select the mushaf by uuid
        // and get the mushaf id
        let mushaf: i32 = quran_mushafs
            .filter(mushaf_uuid.eq(new_surah.mushaf_uuid))
            .select(mushaf_id)
            .get_result(&mut conn)?;

        diesel::update(quran_surahs.filter(surah_uuid.eq(target_surah_uuid)))
            .set((
                number.eq(new_surah.number),
                surah_mushaf_id.eq(mushaf),
                name.eq(new_surah.name),
                bismillah_status.eq(new_surah.bismillah_status),
                bismillah_as_first_ayah.eq(new_surah.bismillah_as_first_ayah),
                period.eq(new_surah.period),
                name_pronunciation.eq(new_surah.name_pronunciation),
                name_translation_phrase.eq(new_surah.name_translation_phrase),
                name_transliteration.eq(new_surah.name_transliteration),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}
