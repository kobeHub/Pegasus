//use actix_session::Session;
use actix_web::{post, web, HttpResponse, Scope};
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::invitation::{Invitation, InvitationData};
use crate::models::user::User;
use crate::services::email_service;

#[post("/post")]
async fn post_invitation(
    invit_data: web::Json<InvitationData>,
    //sess: Session,
) -> Result<HttpResponse, ApiError> {
    /*if let None = sess.get::<Option<ClusterRole>>("cluster_role")? {
            return Err(ApiError::new(401, "Unauthorized".to_string()))
        } else if let Some(ClusterRole::Lessee) = sess.get("cluster_role")? {
            return Ok(HttpResponse::Ok().json(json!({
                "status": false,
                "msg": "You're not allowed to invitate membors",
            })))
    }*/

    let data = invit_data.into_inner();
    // Check user exist
    if User::exist(&data.email)? {
        return Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "msg": "The user with this email exist",
        })));
    }

    // Send limits
    let cnt = Invitation::count_one_day(&data.email)?;
    if cnt >= 3 {
        return Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "msg": "Most 3 invitations are allow for one email within 24 hours"
        })));
    }

    let info = Invitation::create(&data)?;
    email_service::send_email(&info)?;
    Ok(HttpResponse::Ok().json(json!({
           "status": true,
           "msg": format!(
            "Invitation for {} send successfully",
            &data.email
    )})))
}

#[derive(Deserialize)]
struct ExpireInfo {
    pub id: String,
}

#[post("/expire")]
async fn is_expired(info: web::Json<ExpireInfo>) -> Result<HttpResponse, ApiError> {
    let info = Uuid::from_str(&info.id)
        .map_err(|err| ApiError::new(500, format!("Parse uuid: {}", err)))?;

    if Invitation::is_expired(&info)? {
        Ok(HttpResponse::Ok().json(json!({
            "expire": true
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "expire": false
        })))
    }
}

type EmailInfo = ExpireInfo;
#[post("/get")]
async fn get_email(info: web::Json<EmailInfo>) -> Result<HttpResponse, ApiError> {
    let info = Uuid::from_str(&info.id)
        .map_err(|err| ApiError::new(500, format!("Parse uuid: {}", err)))?;

    let res = Invitation::get_info(&info)?;
    Ok(HttpResponse::Ok().json(json!(res)))
}

pub fn invitation_scope() -> Scope {
    web::scope("/invitations")
        .service(post_invitation)
        .service(is_expired)
        .service(get_email)
}
