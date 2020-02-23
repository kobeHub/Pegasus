use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::utils::schema::invitations;
use crate::errors::ApiError;
use super::db;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

/// Any type impl `Into<String>` can create `Invitation`
/// default invitation expires after 24 hours
impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        let now = Utc::now().naive_utc();
        Invitation {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            expires_at: now + chrono::Duration::hours(24),
            created_at: now,
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

    /// Count records within 24 hours
    pub fn count_one_day(eml: &str) ->
        Result<i64, ApiError> {
        let conn = db::connection()?;

        let to = Utc::now().naive_utc();
        let from = to - chrono::Duration::hours(24);
        let results: i64 = invitations::table
                .filter(invitations::email.eq(eml))
                .filter(invitations::created_at.ge(from))
                .filter(invitations::created_at.le(to))
                .count()
                .get_result(&conn)?;
        Ok(results)
        }

    pub fn is_expired(id: &Uuid) -> Result<bool, ApiError> {
        let conn = db::connection()?;

        let now = Utc::now().naive_utc();
        let info: Invitation = invitations::table
            .filter(invitations::id.eq(id))
            .first(&conn)?;
        Ok(info.expires_at < now)
    }
}

/// Struct to hold user sent data
#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}
