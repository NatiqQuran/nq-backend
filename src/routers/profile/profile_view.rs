use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{error::RouterError, models::User, routers::user::FullUserProfile, DbPool};

pub async fn profile_view(
    user_id: web::ReqData<u32>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<FullUserProfile>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, id as account_id, username as account_username, uuid as uuid_of_account,
    };
    use crate::schema::app_emails::dsl::{app_emails, email as email_address};
    use crate::schema::app_user_names::dsl::{
        app_user_names, first_name as f_name, last_name as l_name, primary_name,
    };
    use crate::schema::app_users::dsl::app_users;

    let user_id = user_id.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let (uuid, username, user, email, first_name, last_name): (
            Uuid,
            String,
            User,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = app_accounts
            .inner_join(app_users)
            .left_join(app_emails)
            .left_join(app_user_names)
            .filter(account_id.eq(user_id as i32))
            .filter(primary_name.eq(true).or(primary_name.is_null()))
            .select((
                uuid_of_account,
                // username of the account
                account_username,
                // The User Model
                User::as_select(),
                // User's primary email
                email_address.nullable(),
                // First name from names table
                f_name.nullable(),
                // and last name
                l_name.nullable(),
            ))
            .get_result(&mut conn)?;

        Ok(web::Json(FullUserProfile {
            uuid: uuid.to_string(),
            email,
            username,
            birthday: user.birthday,
            last_name,
            first_name,
            profile_image: user.profile_image,
            language: user.language,
        }))
    })
    .await
    .unwrap()
}
