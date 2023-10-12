use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::SimpleMushaf;

/// Update's single mushaf
pub async fn mushaf_edit(
    path: web::Path<String>,
    new_mushaf: web::Json<SimpleMushaf>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::mushafs::dsl::{
        bismillah_text, mushafs, name as mushaf_name, source as mushaf_source, uuid as mushaf_uuid,
    };

    let new_mushaf = new_mushaf.into_inner();
    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        diesel::update(mushafs.filter(mushaf_uuid.eq(uuid)))
            .set((
                mushaf_name.eq(new_mushaf.name),
                mushaf_source.eq(new_mushaf.source),
                bismillah_text.eq(new_mushaf.bismillah_text),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap();

    result
}
