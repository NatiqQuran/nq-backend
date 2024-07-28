use std::{net::SocketAddr, sync::Arc};

use crate::{
    error::{RouterError, RouterErrorDetail},
    models::Token,
    DbPool,
};
use actix_web::{
    http::{header::HeaderMap, Uri},
    web, ResponseError,
};
use async_trait::async_trait;
use auth_n::{middleware::TokenChecker, HashBuilder};
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
    async fn get_user_id(
        &self,
        req_addr: SocketAddr,
        headers: HeaderMap,
        uri: Uri,
        request_token: &str,
    ) -> Result<u32, Box<dyn ResponseError>> {
        use crate::schema::app_tokens::dsl::*;

        // Token as bytes
        let token_bytes: Vec<u8> = request_token.bytes().collect();

        let token_invalid_error = Box::new(RouterError::from_predefined("AUTHN_TOKEN_INVALID"));

        let mut conn = self.db_pool.get().unwrap();

        let mut error_detail_builder = RouterErrorDetail::builder();

        error_detail_builder
            .request_url(uri.to_string())
            .req_address(req_addr)
            .request_url_parsed(uri.path());

        if let Some(user_agent) = headers.get("User-agent") {
            error_detail_builder.user_agent(user_agent.to_str().unwrap().to_string());
        }

        let error_detail = error_detail_builder.build();

        let token = web::block(move || {
            // Hash the request token
            // Here we use tokengenerator
            // But we can just use sha2
            let hash_builder = HashBuilder::default().set_source(&token_bytes).generate();

            // Selected hashed token from db
            app_tokens
                .filter(token_hash.eq(hash_builder.get_result().unwrap()))
                .load::<Token>(&mut conn)
                .unwrap()
        })
        .await
        .unwrap();

        // Is there any token we found ?
        if token.is_empty() {
            token_invalid_error.log_to_db(Arc::new(self.db_pool.clone()), error_detail);
            return Err(token_invalid_error);
        }

        let last_token = token.first().unwrap();

        error_detail_builder
            .user_token(last_token.token_hash.clone())
            .account_id(last_token.user_id);

        let error_detail = error_detail_builder.build();

        // Return None for teminated token
        if last_token.terminated {
            token_invalid_error.log_to_db(Arc::new(self.db_pool.clone()), error_detail);
            return Err(token_invalid_error);
        }

        Ok(last_token.user_id as u32)
    }

    async fn token_not_found_error(&self) -> Box<dyn ResponseError> {
        Box::new(
            RouterError::from_predefined("AUTHN_TOKEN_NOT_FOUND")
                // TODO: CHECK
                .log_to_db(Arc::new(self.db_pool.clone()), RouterErrorDetail::default()),
        )
    }
}
