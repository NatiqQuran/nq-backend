use crate::error::RouterError;
use crate::filter::Filter;
use crate::models::QuranWord;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

use super::WordListQuery;

/// Returns the list of quran_words
pub async fn word_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<WordListQuery>,
) -> Result<web::Json<Vec<QuranWord>>, RouterError> {
    let pool = pool.into_inner();
    web::block(move || {
        let mut conn = pool.get().unwrap();

        let filtered_words = match QuranWord::filter(Box::from(query)) {
            Ok(filtred) => filtred,
            Err(err) => return Err(err.log_to_db(pool)),
        };

        // Get the list of words from the database
        let words_list = filtered_words.load::<QuranWord>(&mut conn)?;

        Ok(web::Json(words_list))
    })
    .await
    .unwrap()
}
