use actix_utils::future::{ready, Ready};
use actix_web::http::{header, StatusCode};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use actix_web::{HttpMessage, HttpResponse, ResponseError};
use async_trait::async_trait;
use futures_util::future::LocalBoxFuture;
use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;

#[async_trait]
pub trait TokenChecker<T>
where
    T: Sized,
{
    /// This function will return option
    /// if the request token valid return
    /// Some with sized data to pass to the router
    /// otherwise retun None to response with status code 401
    /// Unauthorized
    ///
    /// This function returns the verifyed user ID
    async fn get_user_id(&self, request_token: &str) -> Option<T>
    where
        Self: Sized;
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

#[derive(Clone, Default)]
pub struct TokenAuth<F, Type> {
    authorization_header_required: bool,
    finder: F,
    phantom_type: PhantomData<Type>,
}

impl<F, Type> TokenAuth<F, Type>
where
    F: TokenChecker<Type>,
    Type: Sized,
{
    /// Construct `TokenAuth` middleware.
    pub fn new(finder: F, authorization_header_required: bool) -> Self {
        Self {
            authorization_header_required,
            finder,
            phantom_type: PhantomData,
        }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B, F, T> Transform<S, ServiceRequest> for TokenAuth<F, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    F: TokenChecker<T> + Clone + 'static,
    T: Sized + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TokenAuthMiddleware<S, F, T>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenAuthMiddleware {
            service: Rc::new(service),
            token_finder: self.finder.clone(),
            authorization_header_required: self.authorization_header_required,
            phantom_type: PhantomData,
        }))
    }
}

#[derive(Debug)]
pub struct AccessDeniedError {
    message: &'static str,
}

impl AccessDeniedError {
    pub fn with_message(message: &'static str) -> Self {
        Self { message }
    }
}

impl Display for AccessDeniedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)?;

        Ok(())
    }
}

impl ResponseError for AccessDeniedError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

pub struct TokenAuthMiddleware<S, F, Type> {
    service: Rc<S>,
    token_finder: F,
    authorization_header_required: bool,
    phantom_type: PhantomData<Type>,
}

impl<S, B, F, Type> Service<ServiceRequest> for TokenAuthMiddleware<S, F, Type>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    F: TokenChecker<Type> + Clone + 'static,
    Type: Sized + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let token_finder = self.token_finder.clone();
        let header_required = self.authorization_header_required.clone();

        Box::pin(async move {
            match req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|token| token.to_str().ok())
            {
                Some(token) => {
                    let token_data = token_finder.get_user_id(token).await;

                    if let Some(data) = token_data {
                        req.extensions_mut().insert(data);
                        let res = service.call(req).await?;
                        return Ok(res);
                    };

                    Err(Error::from(AccessDeniedError::with_message(
                        "Unauthorized (Token invalid), Please login again!",
                    )))
                }
                None => {
                    if header_required {
                        return Err(Error::from(AccessDeniedError::with_message(
                            "Unauthorized, Please login!",
                        )));
                    }

                    let res = service.call(req).await?;

                    Ok(res)
                }
            }
        })
    }
}
