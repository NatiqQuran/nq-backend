use crate::error::{RouterError, RouterErrorDetail};
use crate::filter::Filter;
use crate::models::QuranAyah;
use crate::DbPool;
use actix_web::{web, HttpRequest};
use diesel::prelude::*;

use super::AyahListQuery;

/// Returns the list of ayahs
pub async fn ayah_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<AyahListQuery>,
    req: HttpRequest,
) -> Result<web::Json<Vec<QuranAyah>>, RouterError> {
    let pool = pool.into_inner();

    let mut error_detail_builder = RouterErrorDetail::builder();

    let req_ip = req.peer_addr().unwrap();

    error_detail_builder
        .req_address(req_ip)
        .request_url(req.uri().to_string())
        .request_url_parsed(req.uri().path());

    if let Some(user_agent) = req.headers().get("User-agent") {
        error_detail_builder.user_agent(user_agent.to_str().unwrap().to_string());
    }

    let error_detail = error_detail_builder.build();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let filtered_ayahs = match QuranAyah::filter(Box::from(query)) {
            Ok(filtered) => filtered,
            Err(err) => return Err(err.log_to_db(pool, error_detail)),
        };

        // Get the list of ayahs from the database
        let ayah_list = filtered_ayahs.load::<QuranAyah>(&mut conn)?;

        Ok(web::Json(ayah_list))
    })
    .await
    .unwrap()
}
