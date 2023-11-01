use crate::error::RouterError;
use crate::models::QuranWord;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

/// Returns the list of quran_words
pub async fn word_list(pool: web::Data<DbPool>) -> Result<web::Json<Vec<QuranWord>>, RouterError> {
    use crate::schema::quran_words::dsl::*;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of words from the database
        let words_list = quran_words.load::<QuranWord>(&mut conn)?;

        Ok(web::Json(words_list))
    })
    .await
    .unwrap()
}
