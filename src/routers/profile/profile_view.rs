use actix_web::web;

use crate::DbPool;

pub async fn profile_view(
    user_id: web::ReqData<u32>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<FullUserProfile>, RouterError> {
    todo!()
}
