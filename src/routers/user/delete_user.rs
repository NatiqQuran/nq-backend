use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Delete's a single user
pub async fn delete_user(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id, uuid as acc_uuid};
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users};

    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        // Select the account by uuid
        let account_id: i32 = app_accounts
            .filter(acc_uuid.eq(uuid))
            .select(acc_id)
            .get_result(&mut conn)?;

        diesel::delete(app_users.filter(user_acc_id.eq(account_id))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap();

    result
}
