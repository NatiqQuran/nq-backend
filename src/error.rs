use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpMessage, HttpRequest, HttpResponse,
};
use auth_z::ParsedPath;
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};
use ipnetwork::IpNetwork;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use uuid::Error as UuidError;

use crate::{models::NewErrorLog, DbPool, FIXED_ERROR_RESPONSES};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreDefinedResponseError {
    status_code: u16,
    message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreDefinedResponseErrors {
    pub errors: HashMap<String, PreDefinedResponseError>,
}

#[derive(Clone, Debug)]
pub struct RouterError {
    error_name: String,
    error: PreDefinedResponseError,
    detail: Option<String>,
}

#[derive(Clone, Debug)]
pub struct RouterErrorDetail {
    /// Request Address: IPv4
    pub req_address: SocketAddr,

    /// Request Account ID if available
    pub account_id: Option<i32>,

    /// Token recived from Authorization Header
    pub user_token: Option<String>,

    /// User Agent header
    pub user_agent: Option<String>,

    pub request_url: Option<String>,
    pub request_controller: Option<String>,
    pub request_action: Option<String>,
    pub request_id: Option<String>,
    pub request_body: Option<Vec<u8>>,
    pub request_body_content_type: Option<String>,
}

impl Default for RouterErrorDetail {
    fn default() -> Self {
        Self {
            req_address: SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0000),
            account_id: None,
            user_token: None,
            user_agent: None,
            request_url: None,
            request_controller: None,
            request_action: None,
            request_id: None,
            request_body: None,
            request_body_content_type: None,
        }
    }
}

impl RouterErrorDetail {
    pub fn builder() -> RouterErrorDetailBuilder {
        RouterErrorDetailBuilder {
            detail: RouterErrorDetail::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterErrorDetailBuilder {
    detail: RouterErrorDetail,
}

impl RouterErrorDetailBuilder {
    pub fn from_http_request(req: &HttpRequest) -> Self {
        let req_ip = req.peer_addr().unwrap();

        let mut error_detail_builder = RouterErrorDetail::builder();

        error_detail_builder
            .request_url(req.full_url().to_string())
            .req_address(req_ip)
            .request_url_parsed(req.uri().path())
            .request_body_content_type(req.content_type().to_string());

        if let Some(user_agent) = req.headers().get("User-Agent") {
            error_detail_builder.user_agent(user_agent.to_str().unwrap().to_string());
        }

        if let Some(token) = req.headers().get("Authorization") {
            error_detail_builder.user_token(token.to_str().unwrap().to_string());
        }

        error_detail_builder
    }

    pub fn req_address(&mut self, address: SocketAddr) -> &mut Self {
        self.detail.req_address = address;

        self
    }

    pub fn account_id(&mut self, id: i32) -> &mut Self {
        self.detail.account_id = Some(id);

        self
    }

    pub fn user_token(&mut self, token: String) -> &mut Self {
        self.detail.user_token = Some(token);

        self
    }

    pub fn user_agent(&mut self, agent: String) -> &mut Self {
        self.detail.user_agent = Some(agent);

        self
    }

    pub fn request_url(&mut self, url: String) -> &mut Self {
        self.detail.request_url = Some(url);

        self
    }

    /// This will parse the url and set request_controller, request_action, request_id params.
    pub fn request_url_parsed(&mut self, url_path: &str) -> &mut Self {
        let parsed = ParsedPath::from(url_path);

        self.detail.request_controller = parsed.controller;
        self.detail.request_action = parsed.action;
        self.detail.request_id = parsed.id;

        self
    }

    pub fn request_controller(&mut self, controller: String) -> &mut Self {
        self.detail.request_controller = Some(controller);

        self
    }

    pub fn request_action(&mut self, action: String) -> &mut Self {
        self.detail.request_action = Some(action);

        self
    }

    pub fn request_id(&mut self, id: String) -> &mut Self {
        self.detail.request_id = Some(id);

        self
    }

    pub fn request_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.detail.request_body = Some(body);

        self
    }

    pub fn request_body_content_type(&mut self, ty: String) -> &mut Self {
        self.detail.request_body_content_type = Some(ty);

        self
    }

    /// Finilize builder
    ///
    /// returns final RouterErrorDetail
    pub fn build(&mut self) -> RouterErrorDetail {
        self.detail.clone()
    }
}

impl RouterError {
    pub fn from_predefined(error_response_name: &str) -> Self {
        let err_resp = FIXED_ERROR_RESPONSES.get().unwrap();

        Self {
            error: err_resp.errors.get(error_response_name).unwrap().clone(),
            error_name: error_response_name.to_string(),
            detail: None,
        }
    }

    pub fn from_predefined_with_detail(error_response_name: &str, detail: &str) -> Self {
        let err_resp = FIXED_ERROR_RESPONSES.get().unwrap();

        Self {
            error: err_resp.errors.get(error_response_name).unwrap().clone(),
            error_name: error_response_name.to_string(),
            detail: Some(detail.to_string()),
        }
    }

    pub fn log_to_db(&self, pool: Arc<DbPool>, detail: RouterErrorDetail) -> Self {
        use crate::schema::app_error_logs::dsl::app_error_logs;

        let mut conn = pool.get().unwrap();

        NewErrorLog {
            error_name: &self.error_name,
            status_code: self.error.status_code as i32,
            message: &self.error.message,
            detail: self.detail.as_ref(),
            account_id: detail.account_id,
            request_user_agent: detail.user_agent.as_ref(),
            request_ipv4: IpNetwork::from(detail.req_address.ip()),
            request_token: detail.user_token,
            request_body: detail.request_body,
            request_url: detail.request_url,
            request_controller: detail.request_controller,
            request_action: detail.request_action,
            request_id: detail.request_id,
            request_body_content_type: detail.request_body_content_type,
        }
        .insert_into(app_error_logs)
        .execute(&mut conn)
        .unwrap();

        self.clone()
    }
}

impl Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error.message)?;
        Ok(())
    }
}

impl Error for RouterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl ResponseError for RouterError {
    fn error_response(&self) -> HttpResponse {
        let json = match self.detail.clone() {
            Some(detail) => json!({
                "error_name": self.error_name,
                "message": self.error.message,
                "detail": detail
            }),

            None => json!({
                "error_name": self.error_name,
                "message": self.error.message,
            }),
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            // WARINING TODO: Do not allow all Cors
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body(json.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.error.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<DieselError> for RouterError {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::NotFound => Self::from_predefined("NOT_FOUND"),
            DieselError::DatabaseError(kind, _) => Self::from(kind),

            err => {
                error!("InternalError: {:?}", err);

                Self::from_predefined("INTERNAL_ERROR")
            }
        }
    }
}

impl From<DatabaseErrorKind> for RouterError {
    fn from(value: DatabaseErrorKind) -> Self {
        match value {
            DatabaseErrorKind::CheckViolation => Self::from_predefined("INTERNAL_ERROR"),

            err => {
                error!("InternalError: {:?}", err);

                Self::from_predefined("INTERNAL_ERROR")
            }
        }
    }
}

impl From<UuidError> for RouterError {
    fn from(_value: UuidError) -> Self {
        Self::from_predefined("UUID_ERROR")
    }
}
