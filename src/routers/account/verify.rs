use super::{time_deference, MAX_RANDOM_CODE, MIN_RANDOM_CODE};
use crate::error::{RouterError, RouterErrorDetailBuilder};
use crate::models::{Account, Email, NewAccount, NewEmail, NewToken, NewUser, User, VerifyCode};
use crate::schema::app_emails;
use crate::{validate::validate, DbPool};
use actix_web::{web, HttpRequest};
use auth_n::HashBuilder;
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate)]
pub struct VerifyCodeInfo {
    #[validate(email)]
    email: String,

    #[validate(range(min = "MIN_RANDOM_CODE", max = "MAX_RANDOM_CODE"))]
    code: i32,
}

/// Verify verification code that sended to email
/// from /account/sendCode router
pub async fn verify(
    pool: web::Data<DbPool>,
    info: web::Json<VerifyCodeInfo>,
    req: HttpRequest,
) -> Result<String, RouterError> {
    use crate::schema::app_accounts;
    use crate::schema::app_tokens;
    use crate::schema::app_users;
    use crate::schema::app_verify_codes::dsl::*;

    validate(&info.0)?;

    let error_detail = RouterErrorDetailBuilder::from_http_request(&req)
        .request_body(serde_json::to_string(&info.0).unwrap().as_bytes().to_vec())
        .build();

    // If in debug mode then generate a dummy token
    //
    // *do not handle the errors
    if cfg!(debug_assertions) {
        let mut conn = pool.get().unwrap();
        let key = "secret".as_bytes().to_vec();
        let token_hash = HashBuilder::default().set_source(&key).generate();

        diesel::insert_into(app_tokens::dsl::app_tokens)
            .values(NewToken {
                account_id: 1,
                token_hash: &token_hash.get_result().unwrap(),
            })
            .execute(&mut conn)
            .unwrap();

        return Ok(String::from("secret"));
    }
    let pool = pool.into_inner();

    // The release mode
    web::block(move || {
        let mut conn = pool.get().unwrap();

        let last_sended_code = app_verify_codes
            .filter(email.eq(info.clone().email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)?;

        let Some(last_sended_code) = last_sended_code.first() else {
            return Err(RouterError::from_predefined("VERIFY_CODE_NOT_SENDED")
                .log_to_db(pool, error_detail));
        };

        // The code is not correct
        if last_sended_code.code != info.code {
            return Err(
                RouterError::from_predefined("VERIFY_CODE_NOT_VALID").log_to_db(pool, error_detail)
            );
        }

        // The code is already used
        if last_sended_code.status == *"used".to_string() {
            return Err(RouterError::from_predefined("VERIFY_CODE_ALREADY_USED")
                .log_to_db(pool, error_detail));
        }

        // Get the time difference for expireation check
        let diff = time_deference(last_sended_code.created_at);

        if diff.num_seconds() >= 70 {
            // status code 410 => Gone
            // The requested resource is no longer available at the server and no forwarding
            // address is known. This condition is expected to be considered permanent.

            return Err(
                RouterError::from_predefined("VERIFY_CODE_EXPIRED").log_to_db(pool, error_detail)
            );
        }

        // Everything is ok now change code status to used
        diesel::update(&last_sended_code)
            .set(status.eq("used".to_string()))
            .execute(&mut conn)?;

        // Check if user exists
        let user_email = app_emails::dsl::app_emails
            .filter(app_emails::dsl::email.eq(&info.email))
            .load::<Email>(&mut conn)?;

        // If we dont have user with request (email) then create it
        // else return it
        let user: User = if user_email.is_empty() {
            // Create a new account
            let new_account = NewAccount {
                username: &String::from(""),
                account_type: &String::from("user"),
            }
            .insert_into(app_accounts::dsl::app_accounts)
            .get_result::<Account>(&mut conn)?;

            let new_user = NewUser {
                birthday: None,
                account_id: new_account.id,
                language: None,
            }
            .insert_into(app_users::dsl::app_users)
            .get_result::<User>(&mut conn)?;

            NewEmail {
                creator_user_id: new_user.id,
                email: &info.email,
                account_id: new_account.id,
                verified: true,
                primary: false,
                deleted: false,
            }
            .insert_into(app_emails::dsl::app_emails)
            .execute(&mut conn)?;

            // Update the account and set the user name to the
            // u{the new account id}
            diesel::update(&new_account)
                .set(app_accounts::dsl::username.eq(format!("u{}", &new_account.id)))
                .execute(&mut conn)?;

            new_user
        } else {
            let user = app_users::dsl::app_users
                .filter(app_users::dsl::account_id.eq(user_email.first().unwrap().account_id))
                .first::<User>(&mut conn)?;

            user.to_owned()
        };

        // Some salts
        let user_account_id_as_string = user.account_id.to_string();
        let time_as_string = chrono::offset::Utc::now().timestamp().to_string();
        let mut random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();

        // source buffer for token
        let mut source = vec![];

        // append salts to the source
        source.append(&mut user_account_id_as_string.as_bytes().to_vec());
        source.append(&mut random_bytes);
        source.append(&mut time_as_string.as_bytes().to_vec());

        let token = HashBuilder::default().set_source(&source).generate();

        let Some(result) = token.get_result() else {
            return Err(
                RouterError::from_predefined("CANT_GENERATE_TOKEN").log_to_db(pool, error_detail)
            );
        };

        // Hash the token itself
        let token_hash = {
            let result_bytes = result.as_bytes().to_vec();

            token
                .set_source(&result_bytes)
                .generate()
                .get_result()
                //TODO: remove the unwrap
                .unwrap()
        };

        let new_token = NewToken {
            account_id: user.account_id,
            token_hash: &token_hash,
        };

        // Save token to the Db
        diesel::insert_into(app_tokens::dsl::app_tokens)
            .values(&new_token)
            .execute(&mut conn)?;

        Ok(result)
    })
    .await
    .unwrap()
}
