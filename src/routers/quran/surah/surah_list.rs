use super::{SurahListQuery, SurahListResponse};
use crate::filter::Filter;
use crate::models::{QuranAyah, QuranMushaf, QuranSurah};
use crate::schema::quran_ayahs::surah_id;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::dsl::count;
use diesel::prelude::*;

/// Get the lists of surah
pub async fn surah_list(
    query: web::Query<SurahListQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<SurahListResponse>>, RouterError> {
    use crate::schema::mushafs::dsl::{mushafs, name as mushaf_name};
    use crate::schema::quran_surahs::dsl::*;

    let query = query.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Select the specific mushaf
        // and check if it exists
        let mushaf = mushafs
            .filter(mushaf_name.eq(&query.mushaf))
            .get_result::<QuranMushaf>(&mut conn)?;

        let filtered_surahs = QuranSurah::filter(Box::from(query))?;

        // Get the list of surahs from the database
        let surahs = filtered_surahs
            .filter(mushaf_id.eq(mushaf.id))
            .load::<QuranSurah>(&mut conn)?;

        let ayahs = surahs
            .clone()
            .into_iter()
            .map(|s| {
                QuranAyah::belonging_to(&s)
                    .select(count(surah_id))
                    .get_result(&mut conn)
                    //TODO: remove unwrap
                    .unwrap()
            })
            .collect::<Vec<i64>>();

        // now iter over the surahs and bind it with
        // number_of_ayahs
        let surahs = surahs
            .into_iter()
            .zip(ayahs)
            .map(|(surah, number_of_ayahs)| SurahListResponse {
                uuid: surah.uuid,
                name: surah.name,
                number: surah.number,
                period: surah.period,
                number_of_ayahs,
            })
            .collect::<Vec<SurahListResponse>>();

        Ok(web::Json(surahs))
    })
    .await
    .unwrap()
}
