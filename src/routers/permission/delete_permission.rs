use std::str::FromStr;

use crate::{error::RouterError, DbPool};
use actix_web::web::{self, Path};
use diesel::prelude::*;
use uuid::Uuid;

/// Cascade delete permission
pub async fn delete_permission(
    target_permission: Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_permissions::dsl::{app_permissions, uuid as uuid_from_permission};

    let target_permission = target_permission.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let uuid = Uuid::from_str(&target_permission)?;

        diesel::delete(app_permissions.filter(uuid_from_permission.eq(uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
