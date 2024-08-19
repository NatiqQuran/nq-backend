use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::FullUserProfile;
use crate::error::RouterError;
use crate::models::User;
use crate::DbPool;

pub async fn view_user(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<FullUserProfile>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, username as account_username, uuid as uuid_of_account,
    };
    use crate::schema::app_emails::dsl::{app_emails, email as email_address};
    use crate::schema::app_user_names::dsl::{
        app_user_names, first_name as f_name, last_name as l_name, primary_name,
    };
    use crate::schema::app_users::dsl::app_users;

    let requested_account_uuid = path.into_inner();

    // select user form db
    // with user_id
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
            .filter(uuid_of_account.eq(requested_account_uuid))
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
