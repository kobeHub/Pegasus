lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY")
        .unwrap_or_else(|_| "0123".repeat(8));
}
