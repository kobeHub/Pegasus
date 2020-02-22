use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::errors::ApiError;
use crate::models::invitation::{Invitation, InvitationData};
use crate::services::email_service;

pub async fn post_invitation(
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
