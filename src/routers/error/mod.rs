use crate::error::{RouterError, RouterErrorDetail};
use crate::filter::{Filter, Filters, Order};
use crate::models::ErrorLog;
use crate::DbPool;
use actix_web::{web, HttpRequest};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorLogQuery {
    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for ErrorLogQuery {
    fn sort(&self) -> Option<String> {
        self.sort.clone()
    }

    fn order(&self) -> Option<Order> {
        self.order.clone()
    }

    fn from(&self) -> Option<u64> {
        self.from
    }

    fn to(&self) -> Option<u64> {
        self.to
    }
}

pub async fn errors_list(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<ErrorLogQuery>,
    req: HttpRequest,
) -> Result<web::Json<Vec<ErrorLog>>, RouterError> {
    let pool = pool.into_inner();

    let req_ip = req.peer_addr().unwrap();

    let mut error_detail_builder = RouterErrorDetail::builder();

    error_detail_builder
        .request_url(req.uri().to_string())
        .req_address(req_ip)
        .request_url_parsed(req.uri().path());

    if let Some(user_agent) = req.headers().get("User-agent") {
        error_detail_builder.user_agent(user_agent.to_str().unwrap().to_string());
    }

    let error_detail = error_detail_builder.build();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let filtered_logs = match ErrorLog::filter(Box::from(query)) {
            Ok(filtred) => filtred,
            Err(err) => return Err(err.log_to_db(pool, error_detail)),
        };

        // Get the list of words from the database
        let errors_list: Vec<ErrorLog> = filtered_logs.get_results(&mut conn)?;

        Ok(web::Json(errors_list))
    })
    .await
    .unwrap()
}
