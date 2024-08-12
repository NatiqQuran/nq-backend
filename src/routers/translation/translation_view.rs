use crate::error::RouterError;
use crate::models::Translation;
use crate::{DbPool, TranslationAyah, TranslationStatus, ViewableTranslation};
use ::uuid::Uuid;
use actix_web::web;
use diesel::{prelude::*, query_dsl::boxed_dsl::BoxedDsl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TranslationViewQuery {
    surah_uuid: Option<Uuid>,
}

/// Return's a single translation
pub async fn translation_view(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<TranslationViewQuery>,
) -> Result<web::Json<ViewableTranslation>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, id as account_table_id, uuid as account_uuid,
    };
    use crate::schema::mushafs::dsl::{id as mushaf_table_id, mushafs, uuid as mushaf_table_uuid};
    use crate::schema::quran_ayahs::dsl::{ayah_number, quran_ayahs, uuid as ayah_uuid};
    use crate::schema::quran_surahs::dsl::{
        mushaf_id as surah_mushaf_id, number as surah_number, quran_surahs,
        uuid as surah_table_uuid,
    };
    use crate::schema::translations::dsl::{translations, uuid as translation_uuid};
    use crate::schema::translations_text::dsl::{
        text as translation_text, translation_id, translations_text, uuid as translation_text_uuid,
    };

    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the single translation from the database
        let translation: Translation = translations
            .filter(translation_uuid.eq(path))
            .get_result(&mut conn)?;

        let mushaf_uuid: Uuid = mushafs
            .filter(mushaf_table_id.eq(translation.mushaf_id))
            .select(mushaf_table_uuid)
            .get_result(&mut conn)?;

        let translator_account_uuid: Uuid = app_accounts
            .filter(account_table_id.eq(translation.translator_account_id))
            .select(account_uuid)
            .get_result(&mut conn)?;

        let mut ayahs = quran_surahs
            .inner_join(quran_ayahs.left_outer_join(translations_text))
            .internal_into_boxed();

        if let Some(uuid) = query.surah_uuid {
            ayahs = ayahs.filter(surah_table_uuid.eq(uuid));
        }

        let result = ayahs
            .filter(surah_mushaf_id.eq(translation.mushaf_id))
            .filter(
                translation_id
                    .eq(translation.id)
                    .or(translation_id.is_null()),
            )
            .select((
                translation_text.nullable(),
                ayah_uuid,
                ayah_number,
                surah_number,
                translation_text_uuid.nullable(),
            ))
            .get_results::<(Option<String>, Uuid, i32, i32, Option<Uuid>)>(&mut conn)?;

        let mut result_ayahs = vec![];
        let mut status = TranslationStatus::Ok;

        for (text, a_uuid, a_number, s_number, text_uuid) in result {
            if text_uuid.is_none() {
                status = TranslationStatus::Incomplete;
            }
            result_ayahs.push(TranslationAyah {
                uuid: a_uuid,
                text,
                surah_number: s_number as u32,
                number: a_number as u32,
                text_uuid,
            });
        }

        if matches!(status, TranslationStatus::Ok) && !translation.approved {
            status = TranslationStatus::NotApproved;
        }

        Ok(web::Json(ViewableTranslation {
            ayahs: result_ayahs,
            status,
            source: translation.source,
            language: translation.language,
            release_date: translation.release_date,
            mushaf_uuid,
            translator_account_uuid,
        }))
    })
    .await
    .unwrap()
}
