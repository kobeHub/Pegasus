use actix_web::web;
use lazy_static::lazy_static;

use crate::errors::ServiceError;

// Read cookie key from `.env` or use default
lazy_static! {
    pub static ref SECRET_KEY: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
    pub static ref SPARKPOST_API_KEY: String =
        std::env::var("SPARKPOST_API_KEY")
        .expect("You must set SPARKPOST_API_KEY in .env file");
    pub static ref SENDING_EMAIL_ADDRESS: String =
        std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("You must set SENDING_EMAIL_ADDRESS in .env file");
    pub static ref ORGANISE_NAME: String =
        std::env::var("ORGANISE_NAME").unwrap_or("Pegasus".to_owned());
}

// return `ServiceError::BadRequest` if parse json error
lazy_static! {
    pub static ref JSON_PARSE_CONFIG: web::JsonConfig = web::JsonConfig::default()
        .error_handler(|err, _req| { ServiceError::BadRequest(err.to_string()).into() });
}
