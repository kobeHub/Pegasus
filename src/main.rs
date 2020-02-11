#![allow(dead_code)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate derive_more;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use dotenv;
use env_logger;
use listenfd::ListenFd;

mod errors;
mod router;
mod models;
mod mw;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info, actix_server=info");
    env_logger::init();
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let mut listenfd = ListenFd::from_env();
    let pool = models::db::build_pool();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::new("Status:%s  Req:\"%r\" %a Time:%Dms"))
            .wrap(mw::build_session("actix_session", 1))
            .wrap(mw::build_identity(&domain, 1))
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
