use actix_web::{web, post, HttpResponse, Scope};

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

#[post("/admin")]
async fn update_admin(info: web::Json<Department>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    if let None = info.admin {
        return Err(ApiError::new(400, "Admin field must be speficed".to_string()))
    }
    let res = info.set_admin()?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn department_scope() -> Scope {
    web::scope("/departs")
        .service(create_depart)
        .service(update_admin)
}
