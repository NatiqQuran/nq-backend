use crate::error::{RouterError, RouterErrorDetail};
use crate::filter::Filter;
use crate::models::QuranMushaf;
use crate::DbPool;
use actix_web::{web, HttpRequest};
use diesel::prelude::*;

use super::MushafListQuery;

/// Get the lists of mushafs
pub async fn mushaf_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<MushafListQuery>,
    req: HttpRequest,
) -> Result<web::Json<Vec<QuranMushaf>>, RouterError> {
    let pool = pool.into_inner();

    let mut error_detail_builder = RouterErrorDetail::builder();

    let req_ip = req.peer_addr().unwrap();

    error_detail_builder
        .req_address(req_ip)
        .request_url_parsed(req.uri().path());

    if let Some(user_agent) = req.headers().get("User-agent") {
        error_detail_builder.user_agent(user_agent.to_str().unwrap().to_string());
    }

    let error_detail = error_detail_builder.build();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of mushafs from the database
        let quran_mushafs = match QuranMushaf::filter(Box::from(query)) {
            Ok(filtred) => filtred,
            Err(err) => return Err(err.log_to_db(pool, error_detail)),
        }
        .load::<QuranMushaf>(&mut conn)?;

        Ok(web::Json(quran_mushafs))
    })
    .await
    .unwrap()
}
