use super::SimpleWord;
use crate::error::RouterError;
use crate::models::{QuranAyah, QuranWord};
use crate::{Sajdah, AyahWithContent, DbPool};
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single ayah
pub async fn ayah_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<AyahWithContent>, RouterError> {
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs, uuid as mushaf_uuid};
    use crate::schema::quran_ayahs::dsl::{quran_ayahs, uuid as ayah_uuid};
    use crate::schema::quran_surahs::dsl::{
        id as surah_id, mushaf_id as surah_mushaf_id, quran_surahs, uuid as surah_uuid,
    };
    use crate::schema::quran_words::dsl::{ayah_id, id as word_id, quran_words};

    let requested_ayah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the single ayah from the database
        let quran_ayah: QuranAyah = quran_ayahs
            .filter(ayah_uuid.eq(requested_ayah_uuid))
            .get_result(&mut conn)?;

        let surah: (Uuid, i32) = quran_surahs
            .filter(surah_id.eq(quran_ayah.surah_id))
            .select((surah_uuid, surah_mushaf_id))
            .get_result(&mut conn)?;

        let mushaf: Uuid = mushafs
            .filter(mushaf_id.eq(surah.1))
            .select(mushaf_uuid)
            .get_result(&mut conn)?;

        let words: Vec<QuranWord> = quran_words
            .filter(ayah_id.eq(quran_ayah.id))
            .order(word_id.asc())
            .get_results(&mut conn)?;

        let words_simple: Vec<SimpleWord> = words
            .into_iter()
            .map(|word| SimpleWord {
                word: word.word,
                uuid: word.uuid,
            })
            .collect();

        let text = words_simple
            .clone()
            .into_iter()
            .map(|word| word.word)
            .collect::<Vec<String>>()
            .join(" ");

        Ok(web::Json(AyahWithContent {
            uuid: quran_ayah.uuid,
            surah: surah.0,
            mushaf,
            sajdah: Sajdah::from_option_string(quran_ayah.sajdah),
            ayah_number: quran_ayah.ayah_number,
            words: words_simple,
            text,
        }))
    })
    .await
    .unwrap()
}
