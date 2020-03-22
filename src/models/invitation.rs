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
    pub department: Option<i32>,
    pub is_admin: bool,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

/// Any type impl `Into<String>` can create `Invitation`
/// default invitation expires after 24 hours
impl From<&InvitationData> for Invitation
where
{
    fn from(data: &InvitationData) -> Self {
        let now = Utc::now().naive_utc();
        Invitation {
            id: uuid::Uuid::new_v4(),
            email: data.email.clone(),
            department: data.department,
            is_admin: data.is_admin,
            expires_at: now + chrono::Duration::hours(24),
            created_at: now,
        }
    }
}

impl Invitation {
    pub fn create(data: &InvitationData) -> Result<Invitation, ApiError> {
        let conn = db::connection()?;

        let info: Invitation = Invitation::from(data);
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

    pub fn get_info(id: &Uuid) -> Result<Invitation, ApiError> {
        let conn = db::connection()?;

        let info: Invitation = invitations::table
            .filter(invitations::id.eq(id))
            .first(&conn)?;
        Ok(info)
    }

    pub fn set_expire(email: &str) -> Result<(), ApiError> {
        let conn = db::connection()?;

        let _res: Invitation = diesel::update(invitations::table.filter(
            invitations::email.eq(email)))
            .set(invitations::expires_at.eq(Utc::now().naive_utc()))
            .get_result(&conn)?;
        Ok(())
    }
}

/// Struct to hold user sent data
#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
    pub department: Option<i32>,
    pub is_admin: bool,
}
