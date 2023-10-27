use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Delete's a single mushaf
pub async fn mushaf_delete(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::mushafs::dsl::{mushafs, uuid as mushaf_uuid};

    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        // remove mushaf
        diesel::delete(mushafs.filter(mushaf_uuid.eq(uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap();

    result
}
