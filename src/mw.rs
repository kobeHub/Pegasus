use actix_redis::RedisSession;
use time::Duration;

use crate::utils::{SECRET_KEY};

// Use redis as session storge
pub fn redis_session(day: i64, domain: &str) -> RedisSession {
    let redis_host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let redis_port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());

    RedisSession::new(format!("{}:{}", redis_host, redis_port), SECRET_KEY.as_bytes())
        .cookie_max_age(Duration::days(day))
        .cookie_domain(domain)
}
