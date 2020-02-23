use actix_web::{web, post, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;

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
struct ExpireInfo(Uuid);

#[post("/expire")]
async fn is_expired(info: web::Json<ExpireInfo>) -> Result<HttpResponse, ApiError> {
    if Invitation::is_expired(&info.into_inner().0)? {
        Err(ApiError::new(401, "The invitation is expired".to_owned()))
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

pub fn invitation_scope() -> Scope {
    web::scope("/invitaions")
        .service(post_invitation)
        .service(is_expired)
}
