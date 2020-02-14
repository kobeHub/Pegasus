use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::errors::ApiError;
use crate::models::invitation::{Invitation, InvitationData};

pub async fn post_invitation(
    invit_data: web::Json<InvitationData>,
) -> Result<HttpResponse, ApiError> {
    Invitation::create(&invit_data.email).and(Ok(HttpResponse::Ok().json(
        json!({"msg": format!(
            "Invitation for {} send successfully",
            &invit_data.email
        )}))))
}
