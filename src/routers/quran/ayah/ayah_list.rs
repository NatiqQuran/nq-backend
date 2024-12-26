use crate::error::{RouterError, RouterErrorDetailBuilder};
use crate::filter::Filter;
use crate::models::QuranAyah;
use crate::routers::multip;
use crate::{calculate_break, AyahBismillah};
use crate::{
    routers::quran::surah::{AyahTy, Format, SimpleAyah},
    DbPool,
};
use actix_web::{web, HttpRequest};
use diesel::prelude::*;

use super::AyahListQuery;

/// Returns the list of ayahs
pub async fn ayah_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<AyahListQuery>,
    req: HttpRequest,
) -> Result<web::Json<Vec<AyahTy>>, RouterError> {
    use crate::schema::quran_mushafs::dsl::{quran_mushafs, short_name as mushaf_short_name};
    use crate::schema::quran_surahs::dsl::quran_surahs;
    use crate::schema::quran_words::dsl::{quran_words, word as q_word};
    use crate::schema::quran_words_breakers::dsl::{
        quran_words_breakers, word_id as break_word_id,
    };

    use crate::schema::quran_ayahs_breakers::dsl::{
        quran_ayahs_breakers, name as ayah_break_name,
    };

    let pool = pool.into_inner();

    let error_detail = RouterErrorDetailBuilder::from_http_request(&req).build();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let filtered_ayahs = match QuranAyah::filter(Box::from(query.clone())) {
            Ok(filtered) => filtered,
            Err(err) => return Err(err.log_to_db(pool, error_detail)),
        };

        let ayahs = filtered_ayahs
            .left_outer_join(quran_surahs.left_outer_join(quran_mushafs))
            .left_join(quran_ayahs_breakers)
            .inner_join(quran_words.left_join(quran_words_breakers))
            .filter(mushaf_short_name.eq(query.mushaf))
            .select((
                QuranAyah::as_select(),
                q_word,
                break_word_id.nullable(),
                ayah_break_name.nullable(),
            ))
            .get_results::<(QuranAyah, String, Option<i32>, Option<String>)>(&mut conn)?;

        let result = calculate_break(ayahs);

        let ayahs_as_map = multip(result, |a| a);

        let final_ayahs = ayahs_as_map
            .into_iter()
            .map(|(ayah, words)| match query.format {
                Some(Format::Text) | None => AyahTy::Text(crate::AyahWithText {
                    hizb: ayah.hizb,
                    juz: ayah.juz,
                    page: ayah.page,
                    ayah,
                    text: words
                        .into_iter()
                        .map(|w| w.word)
                        .collect::<Vec<String>>()
                        .join(" "),
                }),
                Some(Format::Word) => AyahTy::Words(crate::AyahWithWords { ayah, words }),
            })
            .collect::<Vec<AyahTy>>();

        Ok(web::Json(final_ayahs))
    })
    .await
    .unwrap()
}
