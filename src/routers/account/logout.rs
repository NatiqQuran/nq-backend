use crate::error::RouterError;
use crate::DbPool;
use actix_web::web::{self, ReqData};
use diesel::prelude::*;

/// This will teminate the user token
pub async fn logout<'a>(
    pool: web::Data<DbPool>,
    data: ReqData<u32>,
) -> Result<&'a str, RouterError> {
    use crate::models::Token;
    use crate::schema::app_tokens::dsl::*;

    let req_account_id = data.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the latest token
        let token = app_tokens
            .filter(account_id.eq(req_account_id as i32))
            .order(created_at.desc())
            .limit(1)
            .first::<Token>(&mut conn)?;

        // Now terminate the token
        // Set the terminated to true
        // And set request id to the terminated_by_id
        // This may change.
        diesel::update(&token)
            .set((
                terminated.eq(true),
                terminated_by_id.eq(req_account_id as i32),
            ))
            .execute(&mut conn)?;

        Ok("Logged Out")
    })
    .await
    .unwrap()
}
