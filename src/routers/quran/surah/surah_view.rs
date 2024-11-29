use super::{
    AyahBismillah, Format, GetSurahQuery, QuranResponseData, SimpleAyah, SingleSurahResponse,
};
use crate::models::{QuranAyah, QuranMushaf, QuranSurah};
use crate::routers::multip;
use crate::{error::RouterError, DbPool};
use crate::{AyahTy, SingleSurahMushaf, SurahName};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// View Surah
pub async fn surah_view(
    path: web::Path<Uuid>,
    query: web::Query<GetSurahQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranResponseData>, RouterError> {
    use crate::schema::app_phrase_translations::dsl::{
        app_phrase_translations, language as p_t_lang, text as p_t_text,
    };
    use crate::schema::app_phrases::dsl::{app_phrases, phrase as p_phrase};
    use crate::schema::quran_ayahs::dsl::quran_ayahs;
    use crate::schema::quran_mushafs::dsl::{id as mushaf_id, quran_mushafs};
    use crate::schema::quran_surahs::dsl::quran_surahs;
    use crate::schema::quran_surahs::dsl::uuid as surah_uuid;
    use crate::schema::quran_words::dsl::{quran_words, word as q_word};

    let query = query.into_inner();
    let requested_surah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let result = quran_surahs
            .filter(surah_uuid.eq(requested_surah_uuid))
            .inner_join(quran_ayahs.inner_join(quran_words))
            .select((QuranAyah::as_select(), q_word))
            .load::<(QuranAyah, String)>(&mut conn)?;

        let ayahs_as_map = multip(result, |ayah| SimpleAyah {
            number: ayah.ayah_number as u32,
            uuid: ayah.uuid,
            sajdah: ayah.sajdah,
            bismillah: match (ayah.is_bismillah, ayah.bismillah_text) {
                (true, None) => Some(AyahBismillah {
                    is_ayah: true,
                    text: None,
                }),
                (false, Some(text)) => Some(AyahBismillah {
                    is_ayah: false,
                    text: Some(text),
                }),
                (false, None) => None,
                (_, _) => None,
            },
        });

        let final_ayahs = ayahs_as_map
            .into_iter()
            .map(|(ayah, words)| match query.format {
                Format::Text => AyahTy::Text(crate::AyahWithText {
                    ayah,
                    text: words.join(" "),
                }),
                Format::Word => AyahTy::Words(crate::AyahWithWords { ayah, words }),
            })
            .collect::<Vec<AyahTy>>();

        // Get the surah
        let surah = quran_surahs
            .filter(surah_uuid.eq(requested_surah_uuid))
            .get_result::<QuranSurah>(&mut conn)?;

        // Get the mushaf
        let mushaf = quran_mushafs
            .filter(mushaf_id.eq(surah.mushaf_id))
            .get_result::<QuranMushaf>(&mut conn)?;

        let translation = if let Some(ref phrase) = surah.name_translation_phrase {
            let mut p = app_phrases.left_join(app_phrase_translations).into_boxed();

            if let Some(ref l) = query.lang_code {
                p = p.filter(p_t_lang.eq(l));
            } else {
                p = p.filter(p_t_lang.eq("en"));
            }

            p.filter(p_phrase.eq(phrase))
                .select(p_t_text.nullable())
                .get_result(&mut conn)?
        } else {
            None
        };

        Ok(web::Json(QuranResponseData {
            surah: SingleSurahResponse {
                uuid: surah.uuid,
                mushaf: SingleSurahMushaf::from(mushaf),
                bismillah: final_ayahs.first().unwrap().format_bismillah_for_surah(),
                names: vec![SurahName {
                    arabic: surah.name,
                    translation,
                    translation_phrase: surah.name_translation_phrase,
                    pronunciation: surah.name_pronunciation,
                    transliteration: surah.name_transliteration,
                }],
                period: surah.period,
                number: surah.number,
                number_of_ayahs: final_ayahs.len() as i64,
            },
            ayahs: final_ayahs,
        }))
    })
    .await
    .unwrap()
}
