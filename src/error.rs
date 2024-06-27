use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, error::Error, fmt::Display, sync::Arc};
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

    pub fn log_to_db(&self, pool: Arc<DbPool>) -> Self {
        use crate::schema::app_error_logs::dsl::app_error_logs;

        let mut conn = pool.get().unwrap();

        NewErrorLog {
            error_name: &self.error_name,
            status_code: self.error.status_code as i32,
            message: &self.error.message,
            detail: self.detail.as_ref(),
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
