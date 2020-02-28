use actix_web::{web, post, HttpResponse, Scope};
use serde_json::json;

use crate::errors::ApiError;
use crate::models::department::Department;

#[derive(Deserialize)]
struct Info {
    pub name: String,
}

#[post("/create")]
async fn create_depart(info: web::Json<Info>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner().name;
    let res = Department::create(info)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn department_scope() -> Scope {
    web::scope("/departs")
        .service(create_depart)
}
