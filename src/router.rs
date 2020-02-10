use actix_web::{web, get, HttpResponse, Scope, Result};

#[get("/")]
pub async fn healthy() ->  Result<String> {
    Ok(String::from("Pegasus server is healthy!"))
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .route("/", web::get().to(|| HttpResponse::Ok().body("Pegasus is healthy!\n")))
}
