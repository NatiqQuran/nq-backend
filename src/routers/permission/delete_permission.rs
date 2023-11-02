use crate::{error::RouterError, DbPool};
use actix_web::web::{self, Path};
use diesel::prelude::*;
use uuid::Uuid;

/// Cascade delete permission
pub async fn delete_permission(
    path: Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_permissions::dsl::{app_permissions, uuid as uuid_from_permission};

    let target_permission_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::delete(app_permissions.filter(uuid_from_permission.eq(target_permission_uuid)))
            .execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
