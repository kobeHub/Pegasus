/// Setting the generic error response which allows
/// us send http error with a custom message. Rust provides
/// powerful tools to convert one error type to another one.
///
/// The app doing a few operations using different crates.
/// ie. use `diesel` process ORM, hash password with `bcrypt` etc
/// These operations may returns errors that allowed to be converted into `PegasusError`
/// via `std::convert::From` trait.

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::parser::ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Pegasus Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Pegasus BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Pegasus Unauthorized")]
    Unauthorized,
}

// impl `ResponseError` trait allows `ServiceError` convert to
// http response with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError()
                    .json("Internal Server Error, please try again later")
            },
            ServiceError::BadRequest(ref msg) => {
                HttpResponse::BadRequest()
                    .json(msg)
            },
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
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
                    let msg = info.details()
                        .unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(msg);
                }
                ServiceError::InternalServerError
            },
            _ => ServiceError::InternalServerError
        }
    }
}
