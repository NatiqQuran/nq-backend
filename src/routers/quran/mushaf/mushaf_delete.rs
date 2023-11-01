use crate::error::RouterError;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Delete's a single mushaf
pub async fn mushaf_delete(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::mushafs::dsl::{mushafs, uuid as mushaf_uuid};

    let target_mushaf_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // remove mushaf
        diesel::delete(mushafs.filter(mushaf_uuid.eq(target_mushaf_uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
