use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

pub async fn delete_phrase(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_phrases::dsl::{app_phrases, phrase};

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::delete(app_phrases.filter(phrase.eq(path.into_inner()))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
