use crate::error::RouterError;
use crate::filter::{Filter, Filters, Order};
use crate::models::ErrorLog;
use crate::DbPool;
use actix_web::web;
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
) -> Result<web::Json<Vec<ErrorLog>>, RouterError> {
    let pool = pool.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let filtered_logs = match ErrorLog::filter(Box::from(query)) {
            Ok(filtred) => filtred,
            Err(err) => return Err(err.log_to_db(pool)),
        };

        // Get the list of words from the database
        let errors_list = filtered_logs.load::<ErrorLog>(&mut conn)?;

        Ok(web::Json(errors_list))
    })
    .await
    .unwrap()
}
