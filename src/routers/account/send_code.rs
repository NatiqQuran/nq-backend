use super::{time_deference, MAX_RANDOM_CODE, MIN_RANDOM_CODE};
use crate::email::EmailManager;
use crate::error::{RouterError, RouterErrorDetail};
use crate::models::{NewVerifyCode, VerifyCode};
use crate::validate::validate;
use crate::DbPool;
use actix_web::{web, HttpRequest};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Generate random code with range (min, max)
pub fn generate_random_code(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}

enum SendCodeStatus {
    /// This means we need to send code
    /// to email
    SendCode(String),

    /// This means we already sended code
    /// in past 10 seconds
    AlreadySent,
}

#[derive(Deserialize, Serialize, Clone, Validate)]
pub struct SendCodeInfo {
    #[validate(email)]
    email: String,
}

/// Data -> Email,
/// Send Random generated code to user email
pub async fn send_code(
    pool: web::Data<DbPool>,
    emailer: web::Data<EmailManager>,
    info: web::Json<SendCodeInfo>,
    req: HttpRequest,
) -> Result<String, RouterError> {
    use crate::schema::app_verify_codes::dsl::*;

    validate(&info.0)?;

    let info_copy = info.clone();
    let pool_clone = pool.clone();

    let error_detail = RouterErrorDetail::builder()
        .from_http_request(&req)
        .request_body(serde_json::to_string(&info.0).unwrap().as_bytes().to_vec())
        .build();

    let send_status: Result<SendCodeStatus, RouterError> = web::block(move || {
        let random_code = generate_random_code(MIN_RANDOM_CODE, MAX_RANDOM_CODE);
        let mut conn = pool.get().unwrap();

        // Get last sended code, order by created_at
        let last_sended_code = app_verify_codes
            .filter(email.eq(&info_copy.email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)?;

        // Is there any code we sent ?
        if !last_sended_code.is_empty() {
            let diff = time_deference(last_sended_code[0].created_at);

            // Check if code not expired
            if diff.num_seconds() < 10 {
                return Ok(SendCodeStatus::AlreadySent);
            }
        }

        // Create new code
        let new_code = NewVerifyCode {
            code: &random_code,
            email: &info_copy.email,
            status: &"notUsed".to_string(),
        };

        // Insert code to app_verify_code table
        diesel::insert_into(app_verify_codes)
            .values(&new_code)
            .execute(&mut conn)?;

        Ok(SendCodeStatus::SendCode(random_code.to_string()))
    })
    .await
    .unwrap();

    if let Ok(send_status) = send_status {
        match send_status {
            SendCodeStatus::SendCode(new_code) => {
                let result = emailer
                    .send_email(
                        &info.email,
                        "Verification Code",
                        format!("Code: {}", new_code),
                    )
                    .await;

                match result {
                    Ok(()) => Ok("Code sended".to_string()),
                    Err(_error) => Err(RouterError::from_predefined("SEND_CODE_INTERNAL_ERROR")
                        .log_to_db(pool_clone.into_inner(), error_detail)),
                }
            }
            SendCodeStatus::AlreadySent => Ok("Already sent".to_string()),
        }
    } else {
        Err(send_status.err().unwrap())
    }
}
