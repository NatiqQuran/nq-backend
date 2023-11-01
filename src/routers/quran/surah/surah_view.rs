use super::{
    Ayah, AyahTextType, Format, GetSurahQuery, QuranResponseData, SimpleAyah, SingleSurahResponse,
};
use crate::models::{QuranAyah, QuranMushaf, QuranSurah, QuranWord};
use crate::routers::multip;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;

/// View Surah
pub async fn surah_view(
    path: web::Path<Uuid>,
    query: web::Query<GetSurahQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranResponseData>, RouterError> {
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs};
    use crate::schema::quran_ayahs::dsl::quran_ayahs;
    use crate::schema::quran_surahs::dsl::quran_surahs;
    use crate::schema::quran_surahs::dsl::uuid as surah_uuid;
    use crate::schema::quran_words::dsl::quran_words;

    let query = query.into_inner();
    let requested_surah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let result = quran_surahs
            .filter(surah_uuid.eq(requested_surah_uuid))
            .inner_join(quran_ayahs.inner_join(quran_words))
            .select((QuranAyah::as_select(), QuranWord::as_select()))
            .load::<(QuranAyah, QuranWord)>(&mut conn)?;

        let ayahs_as_map: BTreeMap<SimpleAyah, Vec<QuranWord>> =
            multip(result, |ayah| SimpleAyah {
                number: ayah.ayah_number,
                uuid: ayah.uuid,
                sajdeh: ayah.sajdeh,
            });

        let final_ayahs = ayahs_as_map
            .into_iter()
            .map(|(ayah, words)| Ayah {
                ayah,
                content: match query.format {
                    Format::Text => AyahTextType::Text(
                        words
                            .into_iter()
                            .map(|qword| qword.word)
                            .collect::<Vec<String>>()
                            .join(" "),
                    ),
                    Format::Word => AyahTextType::Words(words),
                },
            })
            .collect::<Vec<Ayah>>();

        // Get the surah
        let surah = quran_surahs
            .filter(surah_uuid.eq(requested_surah_uuid))
            .get_result::<QuranSurah>(&mut conn)?;

        // Get the mushaf
        let mushaf = mushafs
            .filter(mushaf_id.eq(surah.mushaf_id))
            .get_result::<QuranMushaf>(&mut conn)?;

        let mushaf_bismillah_text = if surah.bismillah_as_first_ayah {
            None
        } else {
            mushaf.bismillah_text // this is Option<String>
        };

        Ok(web::Json(QuranResponseData {
            surah: SingleSurahResponse {
                mushaf_uuid: mushaf.uuid,
                mushaf_name: mushaf.name,
                surah_uuid: surah.uuid,
                surah_name: surah.name,
                surah_period: surah.period,
                surah_number: surah.number,
                bismillah_status: surah.bismillah_status,
                bismillah_as_first_ayah: surah.bismillah_as_first_ayah,
                bismillah_text: mushaf_bismillah_text,
                number_of_ayahs: final_ayahs.len() as i64,
            },
            ayahs: final_ayahs,
        }))
    })
    .await
    .unwrap()
}
