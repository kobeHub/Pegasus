use actix_http::http::StatusCode;
use actix_web::error::Error as ActixError;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DBError;
use lettre::smtp::error::Error as SmtpError;
use lettre_email::error::Error as ClientError;
use reqwest::Error as ReqError;
use serde_json::error::Error as SerdeError;
use serde_json::json;
use std::fmt;

use kube::Error as KubeError;

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
        ApiError { status_code, msg }
    }
}

impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        match error {
            DBError::DatabaseError(_, err) => ApiError::new(
                409,
                format!("Database error: {}", err.message().to_string()),
            ),
            DBError::NotFound => ApiError::new(404, "Record not found".to_owned()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<SmtpError> for ApiError {
    fn from(error: SmtpError) -> ApiError {
        ApiError::new(500, format!("Email smtp service: {}", error.to_string()))
    }
}

impl From<ClientError> for ApiError {
    fn from(error: ClientError) -> ApiError {
        ApiError::new(500, format!("Email client service: {}", error.to_string()))
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({"msg": self.msg}))
    }

    fn status_code(&self) -> StatusCode {
        match StatusCode::from_u16(self.status_code) {
            Ok(code) => code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ActixError> for ApiError {
    fn from(error: ActixError) -> ApiError {
        ApiError::new(500, error.to_string())
    }
}

impl From<KubeError> for ApiError {
    fn from(error: KubeError) -> ApiError {
        match error {
            KubeError::Api(ae) => ApiError::new(
                ae.code,
                format!("{}:{}, reason:{}", ae.status, ae.message, ae.reason),
            ),
            KubeError::SerdeError(e) => ApiError::new(500, format!("Kube serde error: {}", e)),
            _ => ApiError::new(500, format!("kube error: {}", error.to_string())),
        }
    }
}

impl From<SerdeError> for ApiError {
    fn from(error: SerdeError) -> ApiError {
        ApiError::new(500, format!("Json serde error: {}", error))
    }
}

impl From<ReqError> for ApiError {
    fn from(error: ReqError) -> Self {
        ApiError::new(500, format!("Reqwest call error: {}", error))
    }
}
