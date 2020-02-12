use actix_web::web;
use lazy_static::lazy_static;

use crate::errors::ServiceError;

// Read cookie key from `.env` or use default
lazy_static! {
    pub static ref SECRET_KEY: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

// return `ServiceError::BadRequest` if parse json error
lazy_static! {
    pub static ref JSON_PARSE_CONFIG: web::JsonConfig = web::JsonConfig::default()
        .error_handler(|err, _req| { ServiceError::BadRequest(err.to_string()).into() });
}
