use crate::{error::RouterError, models::NewTranslation, DbPool};
use actix_web::web;
use diesel::prelude::*;

use super::SimpleTranslation;

// Add's and new translation
pub async fn translation_add(
    new_translation: web::Json<SimpleTranslation>,
    pool: web::Data<DbPool>,
    data: web::ReqData<u32>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as account_id, uuid as account_uuid};
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users, id as user_id};
    use crate::schema::quran_translations::dsl::quran_translations;

    let new_translation = new_translation.into_inner();
    let data = data.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the creator user-id
        let user: i32 = app_users
            .filter(user_acc_id.eq(data as i32))
            .select(user_id)
            .get_result(&mut conn)?;

        // Get the translator id
        let translator_id: i32 = match new_translation.translator_account_uuid {
            // This means creator wants to set the translator id to another user(account)
            Some(uuid) => app_accounts
                .filter(account_uuid.eq(uuid))
                .select(account_id)
                .get_result(&mut conn)?,

            // This means the creator of translation is translator id
            None => data as i32,
        };

        NewTranslation {
            creator_user_id: user,
            translator_account_id: translator_id,
            source: new_translation.source,
            language: new_translation.language,
            release_date: new_translation.release_date,
        }
        .insert_into(quran_translations)
        .execute(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap()
}
