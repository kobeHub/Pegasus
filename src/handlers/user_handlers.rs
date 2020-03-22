use actix_web::{web, post, Scope, HttpResponse};
use actix_session::Session;
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::user::{ClusterRole, User, UserInfo, LoginInfo};
use crate::models::department::Department;
use crate::models::invitation::Invitation;

#[post("/register")]
async fn register(info: web::Json<UserInfo>) -> Result<HttpResponse, ApiError> {
    if User::exist(&info.email)? {
        Ok(HttpResponse::Ok().json(
            json!({
                "status": false,
                "msg":format!("User with {} exists already!", info.email)}
            )))
    } else {
        let user = User::create(info.into_inner())?;
        if let ClusterRole::DepartmentAdmin = user.role {
            Department::set_admin(user.belong_to.unwrap_or(2), &user.id)?;
        }
        Invitation::set_expire(&user.email)?;
        Ok(HttpResponse::Ok().json(json!({
            "status": true,
            "msg": "Sign up successfully!",
        })))
    }
}

#[post("/login")]
async fn sign_in(info: web::Json<LoginInfo>,
                 sess: Session) ->  Result<HttpResponse, ApiError> {
    let credentials = info.into_inner();

    let user = User::find_by_email(&credentials.email)
        .map_err(|err| {
            match err.status_code {
                404 => ApiError::new(401, "User doesn't exists".to_owned()),
                _ => err,
            }
        })?;

    let is_valid = user.verify_password(&credentials.password)?;

    if is_valid {
        sess.set("user_id", user.id)?;
        sess.set("cluster_role", &user.role)?;
        sess.renew();

        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Password not invalid".to_owned()))
    }
}

#[post("/logout")]
async fn sign_out(sess: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = sess.get("user_id")?;

    if let Some(_) = id {
        sess.purge();
        Ok(HttpResponse::Ok().json(json!({
            "msg": "Signed out successfully"
        })))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/whoami")]
async fn who_am_i(sess: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = sess.get("user_id")?;

    if let Some(id) = id {
        let user = User::find(id)?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_owned()))
    }
}

pub fn user_scope() -> Scope {
    web::scope("/users")
        .service(register)
        .service(sign_in)
        .service(sign_out)
        .service(who_am_i)
}
