use actix_web::{web, get, HttpResponse, Scope, Result};
use actix_session::{Session};

#[get("/")]
pub async fn healthy() ->  Result<String> {
    Ok(String::from("Pegasus server is healthy!"))
}

async fn sess_usage(session: Session) -> Result<HttpResponse> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?
    } else {
        session.set("counter", 1)?
    }

    Ok(HttpResponse::Ok().body(
        format!("Access count: {:?}", session.get::<i32>("counter")?.unwrap())
    ))
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .route("/", web::get().to(|| HttpResponse::Ok().body("Pegasus is healthy!\n")))
        .route("/sess", web::get().to(sess_usage))
}
