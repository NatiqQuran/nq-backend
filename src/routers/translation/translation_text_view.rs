use crate::error::RouterError;
use crate::models::TranslationText;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TextViewQuery {
    pub ayah_uuid: Uuid,
}

/// Return's a single translation_text
pub async fn translation_text_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
    query: web::Query<TextViewQuery>,
) -> Result<web::Json<TranslationText>, RouterError> {
    use crate::schema::quran_ayahs::dsl::{id as ayah_id, quran_ayahs, uuid as ayah_uuid};
    use crate::schema::translations::dsl::{
        id as translations_id, translations, uuid as translation_uuid,
    };
    use crate::schema::translations_text::dsl::{
        ayah_id as text_ayah_id, translation_id as text_translation_id, translations_text,
    };

    let path = path.into_inner();
    let query = query.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the translation by uuid
        let translation: i32 = translations
            .filter(translation_uuid.eq(path))
            .select(translations_id)
            .get_result(&mut conn)?;

        // Get the ayah by uuid
        let ayah: i32 = quran_ayahs
            .filter(ayah_uuid.eq(query.ayah_uuid))
            .select(ayah_id)
            .get_result(&mut conn)?;

        // Get the single translation_text from the database
        let translation_text: TranslationText = translations_text
            .filter(text_ayah_id.eq(ayah))
            .filter(text_translation_id.eq(translation))
            .get_result(&mut conn)?;

        Ok(web::Json(translation_text))
    })
    .await
    .unwrap();

    result
}
