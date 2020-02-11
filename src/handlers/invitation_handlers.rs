use actix_web::{error::BlockingError, web, HttpResponse, Result};
use diesel::prelude::*;
use futures::Future;

use crate::errors::ServiceError;
use crate::models::Pool;
use crate::models::invitation::Invitation;

pub async fn post_invitation(invit_data: web::Json<InvitationData>,
                             pool: web::Data<Pool>)
                             -> Result<HttpResponse, ServiceError> {
    // run diesel block mode
    web::block(move || create_invitation(invit_data.email, pool)
               .and_then(|_| Ok(HttpResponse::Ok().json("Invitations send successfully")))
               .or_else(|err| {
                   match err {
                       BlockingError::Error(service_error) => Err(service_error),
                       BlockingError::Canceled => Err(ServiceError::InternalServerError),
                   }
               }))
}

// Query info and send invitations
fn create_invitation(eml: String, pool: web::Data<Pool>)
                     -> Result<(), BlockingError<ServiceError>> {
    let invitation = dbg!(query(eml, pool)?);

    Ok(())
}

// Diesel query
fn query(eml: String, pool: web::Data<Pool>)
         -> Result<Invitation, ServiceError> {
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
