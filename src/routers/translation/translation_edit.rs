use crate::error::RouterError;
use crate::{DbPool, EditableSimpleTranslation};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// Update's single Translation
pub async fn translation_edit(
    path: web::Path<Uuid>,
    new_translation: web::Json<EditableSimpleTranslation>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::translations::dsl::{
        bismillah_text as translation_bismillah_text, language as translation_language,
        release_date as translation_release_date, source as translation_source, translations,
        uuid as translation_uuid,
    };

    let new_translation = new_translation.into_inner();
    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::update(translations.filter(translation_uuid.eq(path)))
            .set((
                translation_source.eq(new_translation.source),
                translation_release_date.eq(new_translation.release_date),
                translation_language.eq(new_translation.language),
                translation_bismillah_text.eq(new_translation.bismillah_text),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}
