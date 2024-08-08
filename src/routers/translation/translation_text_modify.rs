use crate::translation_text_view::TextViewQuery;
use crate::{error::RouterError, models::NewTranslationText, DbPool};
use actix_web::web;
use diesel::select;
use diesel::{dsl::exists, prelude::*};
use uuid::Uuid;

use super::SimpleTranslationText;

/// Modify translation text,
///
/// If the translation to an ayah exists updated it,
/// otherwise add.
pub async fn translation_text_modify(
    new_translation_text: web::Json<SimpleTranslationText>,
    pool: web::Data<DbPool>,
    data: web::ReqData<u32>,
    // translatio uuid
    path: web::Path<Uuid>,
    query: web::Query<TextViewQuery>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users, id as user_id};
    use crate::schema::quran_ayahs::dsl::{id as ayah_id, quran_ayahs, uuid as ayah_uuid};
    use crate::schema::translations::dsl::{
        id as translation_id, translations, uuid as translation_uuid,
    };
    use crate::schema::translations_text::dsl::{
        ayah_id as text_ayah_id, text as text_content, translation_id as text_translation_id,
        translations_text,
    };

    let new_translation_text = new_translation_text.into_inner();
    let path = path.into_inner();
    let creator_id = data.into_inner();
    let query = query.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the target translation
        let translation: i32 = translations
            .filter(translation_uuid.eq(path))
            .select(translation_id)
            .get_result(&mut conn)?;

        // Get the translation text ayah id
        let ayah: i32 = quran_ayahs
            .filter(ayah_uuid.eq(query.ayah_uuid))
            .select(ayah_id)
            .get_result(&mut conn)?;

        // Now check if the translation_text exists
        let text: bool = select(exists(
            translations_text
                .filter(text_ayah_id.eq(ayah))
                .filter(text_translation_id.eq(translation)),
        ))
        .get_result(&mut conn)?;

        if text {
            // This means the translation_text exists, we just need to update it
            diesel::update(translations_text)
                .filter(text_ayah_id.eq(ayah))
                .filter(text_translation_id.eq(translation))
                .set((text_content.eq(new_translation_text.text),))
                .execute(&mut conn)?;

            Ok("Updated")
        } else {
            // Get the userId from users account id
            let user: i32 = app_users
                .filter(user_acc_id.eq(creator_id as i32))
                .select(user_id)
                .get_result(&mut conn)?;

            // This means user wants to add a new translation_text
            NewTranslationText {
                creator_user_id: user,
                text: &new_translation_text.text,
                translation_id: translation,
                ayah_id: ayah,
            }
            .insert_into(translations_text)
            .execute(&mut conn)?;

            Ok("Added")
        }
    })
    .await
    .unwrap()
}
