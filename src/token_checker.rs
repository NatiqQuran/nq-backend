use std::sync::Arc;

use crate::{error::RouterError, models::Token, DbPool};
use actix_web::{web, ResponseError};
use async_trait::async_trait;
use auth_n::token::{HashBuilder, TokenChecker};
use diesel::prelude::*;

/// Returns the token selected
/// from database
#[derive(Clone)]
pub struct UserIdFromToken {
    db_pool: DbPool,
}

impl UserIdFromToken {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl TokenChecker<u32> for UserIdFromToken {
    async fn get_user_id(&self, request_token: &str) -> Result<u32, Box<dyn ResponseError>> {
        use crate::schema::app_tokens::dsl::*;

        // Token as bytes
        let token_bytes: Vec<u8> = request_token.bytes().collect();

        let token_invalid_error = Box::new(RouterError::from_predefined("AUTHN_TOKEN_INVALID"));

        let mut conn = self.db_pool.get().unwrap();

        let token = web::block(move || {
            // Hash the request token
            // Here we use tokengenerator
            // But we can just use sha2
            let hash_builder = HashBuilder::new().set_source(&token_bytes).generate();

            // Selected hashed token from db
            let token = app_tokens
                .filter(token_hash.eq(hash_builder.get_result().unwrap()))
                .load::<Token>(&mut conn)
                .unwrap();

            token
        })
        .await
        .unwrap();

        // Is there any token we found ?
        if token.is_empty() {
            token_invalid_error.log_to_db(Arc::new(self.db_pool.clone()));
            return Err(token_invalid_error);
        }

        let last_token = token.get(0).unwrap();

        // Return None for teminated token
        if last_token.terminated {
            token_invalid_error.log_to_db(Arc::new(self.db_pool.clone()));
            return Err(token_invalid_error);
        }

        Ok(last_token.user_id as u32)
    }

    async fn token_not_found_error(&self) -> Box<dyn ResponseError> {
        Box::new(
            RouterError::from_predefined("AUTHN_TOKEN_NOT_FOUND")
                .log_to_db(Arc::new(self.db_pool.clone())),
        )
    }
}
