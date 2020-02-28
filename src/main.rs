#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_enum;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate log;

extern crate derive_more;
extern crate argon2;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use dotenv;
use env_logger;
use listenfd::ListenFd;

mod errors;
mod handlers;
mod models;
mod mw;
mod router;
mod services;
mod utils;

use utils::DOMAIN;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var(
        "RUST_LOG",
        "actix_web=info,actix_server=info,service_error=debug,kube=trace",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    models::db::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("Status:%s  Req:\"%r\" %a Time:%Dms"))
            .wrap(mw::redis_session(1, DOMAIN.as_str()))
            .service(router::healthy)
            .service(router::api_scope())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.shutdown_timeout(2).run().await
}
