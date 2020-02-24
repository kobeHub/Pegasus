use actix_web::{web, post, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;
use std::str::FromStr;

use crate::errors::ApiError;
use crate::models::invitation::{Invitation, InvitationData};
use crate::services::email_service;

#[post("/post")]
async fn post_invitation(
    invit_data: web::Json<InvitationData>,
) -> Result<HttpResponse, ApiError> {
    let cnt = Invitation::count_one_day(&invit_data.email)?;
    if cnt >= 3 {
        return Ok(HttpResponse::Ok().json(json!({
            "msg": "Most 3 invitations are allow for one email within 24 hours"
        })));
    }
    let info = Invitation::create(&invit_data.email)?;
    email_service::send_email(&info)?;
    Ok(HttpResponse::Ok().json(
          json!({"msg": format!(
             "Invitation for {} send successfully",
             &invit_data.email
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
#[post("/email")]
async fn get_email(info: web::Json<EmailInfo>) -> Result<HttpResponse, ApiError> {
    let info = Uuid::from_str(&info.id)
        .map_err(|err| ApiError::new(500, format!("Parse uuid: {}", err)))?;

    let res: String = Invitation::get_email(&info)?;
    Ok(HttpResponse::Ok().json(json!({
        "email": res
    })))
}

pub fn invitation_scope() -> Scope {
    web::scope("/invitations")
        .service(post_invitation)
        .service(is_expired)
        .service(get_email)
}
