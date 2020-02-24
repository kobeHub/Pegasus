/// Setting the generic error response which allows
/// us send http error with a custom message. Rust provides
/// powerful tools to convert one error type to another one.
///
/// The app doing a few operations using different crates.
/// ie. use `diesel` process ORM, hash password with `bcrypt` etc
/// These operations may returns errors that allowed to be converted into `PegasusError`
/// via `std::convert::From` trait.
use actix_http::ResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use serde_json::json;
use std::convert::From;
use uuid::parser::ParseError;

use failure::Fail;

#[derive(Debug, Display, Fail)]
#[fail(display = "service error")]
pub enum ServiceError {
    #[display(fmt = "Pegasus Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Pegasus BadRequest: {}", _0)]
    BadRequest(String),

    #[allow(dead_code)]
    #[display(fmt = "Pegasus Unauthorized")]
    Unauthorized,
}

// impl `ResponseError` trait allows `ServiceError` convert to
// http response with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(json!({"msg": self.to_string()}))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

/// Early error response in handlers id UUID provides
/// by users is not valid
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("invalid UUID".into())
    }
}

/// User DB operates error
impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let msg = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(msg);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
