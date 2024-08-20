use super::FullUserProfile;
use crate::models::User;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the list of all users
pub async fn users_list(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<FullUserProfile>>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, username as account_username, uuid as uuid_of_account,
    };
    use crate::schema::app_emails::dsl::{app_emails, email as email_address};
    use crate::schema::app_user_names::dsl::{
        app_user_names, first_name as f_name, last_name as l_name, primary_name,
    };
    use crate::schema::app_users::dsl::app_users;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // What is this :|
        // I know this is ugly but
        // this is the best way to make query in this situation
        //
        // good luck if you gonna read this :)
        let users: Vec<(Uuid, String, User, Option<String>, Option<String>, Option<String>)> = app_users
            .inner_join(
                // Join the accounts, emails and user_names
                // tables together
                app_accounts
                    .left_join(app_emails)
                    .left_join(app_user_names),
            )
            // We only want the primary user name
            .filter(primary_name.eq(true).or(primary_name.is_null()))
            .select((
                // select the uuid of account
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
            .load(&mut conn)?;

        let users: Vec<FullUserProfile> = users
            .into_iter()
            .map(
                |(uuid, username, user, email, first_name, last_name)| FullUserProfile {
                    uuid: uuid.to_string(),
                    email,
                    username,
                    birthday: user.birthday,
                    last_name,
                    first_name,
                    profile_image: user.profile_image,
                    language: user.language,
                },
            )
            .collect();

        Ok(web::Json(users))
    })
    .await
    .unwrap()
}
