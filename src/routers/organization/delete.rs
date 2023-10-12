use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Delete's a single organization
pub async fn delete_organization(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id, uuid as account_uuid};
    use crate::schema::app_organizations::dsl::{account_id as org_acc_id, app_organizations};

    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        // Select the Account by uuid
        let account_id: i32 = app_accounts
            .filter(account_uuid.eq(uuid))
            .select(acc_id)
            .get_result(&mut conn)?;

        // remove uuid
        diesel::delete(app_organizations.filter(org_acc_id.eq(account_id))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap();

    result
}
