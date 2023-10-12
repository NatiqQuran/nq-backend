use crate::error::RouterError;
use crate::models::{NewQuranMushaf, User};
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

use super::SimpleMushaf;

/// Add's new mushaf
pub async fn mushaf_add(
    new_mushaf: web::Json<SimpleMushaf>,
    pool: web::Data<DbPool>,
    data: web::ReqData<u32>,
) -> Result<&'static str, RouterError> {
    use crate::schema::mushafs::dsl::mushafs;
    use crate::schema::app_users::dsl::{app_users, account_id as user_acc_id};

    let new_mushaf = new_mushaf.into_inner();
    let data = data.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        let user: User = app_users.filter(user_acc_id.eq(data as i32)).get_result(&mut conn)?;

        NewQuranMushaf {
            creator_user_id: user.id,
            name: Some(&new_mushaf.name),
            source: Some(&new_mushaf.source),
            bismillah_text: new_mushaf.bismillah_text,
        }
        .insert_into(mushafs)
        .execute(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap();

    result
}
