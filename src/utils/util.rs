use actix_web::web;
use lazy_static::lazy_static;

use crate::errors::ServiceError;

// Read cookie key from `.env` or use default
lazy_static! {
    pub static ref SECRET_KEY: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
    pub static ref SENDING_EMAIL_ADDRESS: String = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("You must set SENDING_EMAIL_ADDRESS in .env file");
    pub static ref SENDING_EMAIL_PASSWD: String = std::env::var("SENDING_EMAIL_PASSWD")
        .expect("SENDING_EMAIL_PASSWD must be specified in .env file");
    pub static ref SMTP_SERVER_ADDR: String =
        std::env::var("SMTP_SERVER_ADDR").expect("SMTP_SERVER_ADDR must be specified in .env file");
    pub static ref ORGANISE_NAME: String =
        std::env::var("ORGANISE_NAME").unwrap_or("Pegasus".to_owned());
    pub static ref DOMAIN: String =
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    pub static ref EMAIL_DOMAIN: String =
        std::env::var("EMAIL_DOMAIN").expect("Please spefic the email domain");
    pub static ref ENGINE_API: String =
        std::env::var("ENGINE_API").unwrap_or("http://localhost:9120/api/v1".to_string());
    pub static ref GITHUB_AK: String = std::env::var("GITHUB_AK").expect("GITHUB_AK must be set");
    pub static ref GITHUB_API: String =
        std::env::var("GITHUB_API").expect("GITHUB_API must be set");
    pub static ref GITHUB_OWNER: String =
        std::env::var("GITHUB_OWNER").expect("GITHUB_OWNER must be set");
    pub static ref GITHUB_REPO: String =
        std::env::var("GITHUB_REPO").expect("GITHUB_REPO must be set");
}

// return `ServiceError::BadRequest` if parse json error
lazy_static! {
    pub static ref JSON_PARSE_CONFIG: web::JsonConfig = web::JsonConfig::default()
        .error_handler(|err, _req| { ServiceError::BadRequest(err.to_string()).into() });
}
