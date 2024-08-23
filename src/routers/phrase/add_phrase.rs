use actix_web::web;
use diesel::prelude::*;

use crate::{error::RouterError, models::NewPhrase, DbPool};

use super::NewReqPhrase;

pub async fn add_phrase(
    pool: web::Data<DbPool>,
    web::Json(new_phrase): web::Json<NewReqPhrase>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_phrases::dsl::app_phrases;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        NewPhrase {
            phrase: &new_phrase.phrase,
        }
        .insert_into(app_phrases)
        .execute(&mut conn)?;

        Ok("added")
    })
    .await
    .unwrap()
}
