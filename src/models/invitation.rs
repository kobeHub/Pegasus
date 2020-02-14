use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::utils::schema::invitations;
use crate::errors::ApiError;
use super::db;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}

/// Any type impl `Into<String>` can create `Invitation`
/// default invitation expires after 24 hours
impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            expires_at: Utc::now().naive_utc() + chrono::Duration::hours(24),
        }
    }
}

impl Invitation {
    pub fn create(eml: &str) -> Result<Invitation, ApiError> {
        let conn = db::connection()?;

        let info: Invitation = eml.into();
        let inserted = diesel::insert_into(invitations::table)
            .values(&info)
            .get_result(&conn)?;

        Ok(inserted)
    }
}

/// Struct to hold user sent data
#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}
