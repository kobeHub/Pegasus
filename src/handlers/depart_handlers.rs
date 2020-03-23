use actix_web::{web, post, get, HttpResponse, Scope};

use crate::errors::ApiError;
use crate::models::department::Department;

#[derive(Deserialize)]
struct Info {
    pub name: String,
    pub email: Option<String>,
}

#[post("/create")]
async fn create_depart(info: web::Json<Info>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();
    let res = Department::create(info.name, info.email)?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/admin")]
async fn update_admin(info: web::Json<Department>) -> Result<HttpResponse, ApiError> {
    let info = info.into_inner();

    if let None = info.admin {
        return Err(ApiError::new(400, "Admin field must be speficed".to_string()))
    }
    let res = Department::set_admin(info.id, &info.admin.unwrap())?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/get")]
async fn get_all() -> Result<HttpResponse, ApiError> {
    let results = Department::list_all()?;

    Ok(HttpResponse::Ok().json(results))
}

#[get("/list")]
async fn list_info() -> Result<HttpResponse, ApiError> {
    let results = Department::list_infos()?;

    Ok(HttpResponse::Ok().json(results))
}

pub fn department_scope() -> Scope {
    web::scope("/departs")
        .service(create_depart)
        .service(update_admin)
        .service(get_all)
        .service(list_info)
}
