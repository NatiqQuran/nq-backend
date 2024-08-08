use actix_web::web;
use diesel::prelude::*;

use crate::{
    error::RouterError,
    models::{Account, NewAccount, NewEmail, NewUser, NewUserNames, User},
    DbPool,
};

use super::EditableUser;

pub async fn add_user(
    pool: web::Data<DbPool>,
    req_data: web::ReqData<u32>,
    new_user: web::Json<EditableUser>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::app_accounts;
    use crate::schema::app_accounts::dsl::username;
    use crate::schema::app_emails::dsl::app_emails;
    use crate::schema::app_user_names::dsl::app_user_names;
    use crate::schema::app_users::dsl::app_users;

    let data = new_user.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Create a new account
        let new_account = NewAccount {
            username: &String::from(""),
            account_type: &String::from("user"),
        }
        .insert_into(app_accounts)
        .get_result::<Account>(&mut conn)?;

        NewUser {
            birthday: Some(data.birthday),
            account_id: new_account.id,
            language: Some(data.language.clone()),
        }
        .insert_into(app_users)
        .get_result::<User>(&mut conn)?;

        NewEmail {
            creator_user_id: req_data.clone().into_inner() as i32,
            email: &data.primary_email,
            account_id: new_account.id,
            verified: true,
            primary: false,
            deleted: false,
        }
        .insert_into(app_emails)
        .execute(&mut conn)?;

        // Update the account and set the user name to the
        // u{the new account id}
        diesel::update(&new_account)
            .set(username.eq(format!("u{}", &new_account.id)))
            .execute(&mut conn)?;

        NewUserNames {
            language: Some(data.language),
            last_name: Some(data.last_name),
            first_name: Some(data.first_name),
            account_id: new_account.id,
            primary_name: true,
            creator_user_id: req_data.into_inner() as i32,
        }
        .insert_into(app_user_names)
        .execute(&mut conn)?;

        Ok("added")
    })
    .await
    .unwrap()
}
