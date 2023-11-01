use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::SimpleAyah;

/// Update's single ayah
pub async fn ayah_edit(
    path: web::Path<Uuid>,
    new_ayah: web::Json<SimpleAyah>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::quran_ayahs::dsl::{
        ayah_number, quran_ayahs, sajdeh as ayah_sajdeh, surah_id as ayah_surah_id,
        uuid as ayah_uuid,
    };
    use crate::schema::quran_surahs::dsl::{id as surah_id, quran_surahs, uuid as surah_uuid};

    let new_ayah = new_ayah.into_inner();
    let target_ayah_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the target surah by surah-uuid
        let target_surah: i32 = quran_surahs
            .filter(surah_uuid.eq(Uuid::from_str(&new_ayah.surah_uuid)?))
            .select(surah_id)
            .get_result(&mut conn)?;

        let new_sajdeh = match new_ayah.sajdeh {
            Some(sajdeh) => Some(sajdeh.to_string()),
            None => None,
        };

        diesel::update(quran_ayahs.filter(ayah_uuid.eq(target_ayah_uuid)))
            .set((
                ayah_number.eq(new_ayah.ayah_number),
                ayah_surah_id.eq(target_surah),
                ayah_sajdeh.eq(new_sajdeh),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}
