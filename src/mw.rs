use actix_session::CookieSession;
use actix_identity::{CookieIdentityPolicy, IdentityService};

use crate::utils;

// Set session middleware
pub fn build_session(name: &str, day: i64) -> CookieSession {
    CookieSession::private(utils::SECRET_KEY.as_bytes())
        .name(name)
        .http_only(false)
        .path("/")
        .max_age(day * 24 * 60 * 60)
        .secure(false)
}

// Build identity service middleware
pub fn build_identity(domain: &str, day: i64) -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
            .name("auth")
            .path("/")
            .domain(domain)
            .max_age(day * 24 * 60 * 60)
            .secure(false)
    )
}
