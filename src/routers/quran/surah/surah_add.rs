use std::str::FromStr;

use super::SimpleSurah;
use crate::models::{NewQuranSurah, User};
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

// Add's and new surah
pub async fn surah_add(
    new_surah: web::Json<SimpleSurah>,
    pool: web::Data<DbPool>,
    data: web::ReqData<u32>,
) -> Result<&'static str, RouterError> {
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs, uuid as mushaf_uuid};
    use crate::schema::quran_surahs::dsl::quran_surahs;
    use crate::schema::app_users::dsl::{app_users, account_id as user_acc_id};

    let new_surah = new_surah.into_inner();
    let data = data.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&new_surah.mushaf_uuid)?;

        // Select the mushaf by uuid
        // and get the mushaf id
        let mushaf: i32 = mushafs
            .filter(mushaf_uuid.eq(uuid))
            .select(mushaf_id)
            .get_result(&mut conn)?;

        // Calculate amount of surahs in mushaf
        let latest_surah_number: i64 = quran_surahs
            .inner_join(mushafs)
            .filter(mushaf_id.eq(mushaf))
            .count()
            .get_result(&mut conn)?;

        let user: User = app_users.filter(user_acc_id.eq(data as i32)).get_result(&mut conn)?;

        // Add a new surah
        NewQuranSurah {
            creator_user_id: user.id,
            name: new_surah.name,
            period: new_surah.period,
            number: (latest_surah_number + 1) as i32,
            mushaf_id: mushaf,
            bismillah_status: new_surah.bismillah_status,
            bismillah_as_first_ayah: new_surah.bismillah_as_first_ayah,
        }
        .insert_into(quran_surahs)
        .execute(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap()
}
