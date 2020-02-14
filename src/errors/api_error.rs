use actix_http::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use diesel::result::Error as DBError;
use serde_json::json;

use std::convert::From;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub msg: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl ApiError {
    pub fn new(status_code: u16, msg: String) -> ApiError {
        ApiError {
            status_code,
            msg,
        }
    }
}

impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        match error {
            DBError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DBError::NotFound => ApiError::new(404, "Record not found".to_owned()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({"msg": self.msg}))
    }

    fn status_code(&self) -> StatusCode {
        match StatusCode::from_u16(self.status_code) {
            Ok(code) => code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
