use actix_session::Session;
use actix_web::{get, web, HttpResponse, Result, Scope};

use crate::handlers::{
    depart_handlers, invitation_handlers, kube_test_handlers, ns_handlers, repos_handlers,
    tasks_handlers, user_handlers, ing_handlers,
};
use crate::utils::JSON_PARSE_CONFIG;

#[get("/")]
pub async fn healthy() -> Result<String> {
    Ok(String::from("Pegasus server is healthy!"))
}

async fn sess_usage(session: Session) -> Result<HttpResponse> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?
    } else {
        session.set("counter", 1)?
    }

    Ok(HttpResponse::Ok().body(format!(
        "Access count: {:?}",
        session.get::<i32>("counter")?.unwrap()
    )))
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        // Early Reponse to json parse error
        .app_data(JSON_PARSE_CONFIG.clone())
        .route(
            "/",
            web::get().to(|| HttpResponse::Ok().body("Pegasus is healthy!\n")),
        )
        .route("/sess", web::get().to(sess_usage))
        .service(invitation_handlers::invitation_scope())
        .service(user_handlers::user_scope())
        .service(depart_handlers::department_scope())
        .service(kube_test_handlers::kube_test_scope())
        .service(ns_handlers::ns_scope())
        .service(tasks_handlers::tasks_scope())
        .service(repos_handlers::repos_scope())
        .service(ing_handlers::ing_scope())
}
