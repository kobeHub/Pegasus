use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::errors::ServiceError;
use crate::models::invitation::Invitation;
use crate::models::Pool;

pub async fn post_invitation(
    invit_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // run diesel block mode
    // web::block(move || create_invitation(invit_data.email, pool)?
    //           .and_then(|_| Ok(HttpResponse::Ok().json("Invitations send successfully")))
    //           ).await

    create_invitation(&invit_data.email, pool).and(Ok(HttpResponse::Ok().json(format!(
        "Invitation for {} send successfully",
        &invit_data.email
    ))))
}

// Query info and send invitations
fn create_invitation(eml: &str, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    let invitation = dbg!(query(eml, pool)?);

    Ok(())
}

// Diesel query
fn query(eml: &str, pool: web::Data<Pool>) -> Result<Invitation, ServiceError> {
    use crate::utils::schema::invitations::dsl::invitations;

    let info: Invitation = eml.into();
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted = diesel::insert_into(invitations)
        .values(&info)
        .get_result(conn)?;

    Ok(inserted)
}

/// Struct to hold user sent data
#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}
