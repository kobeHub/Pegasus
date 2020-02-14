use argon2::{Config};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::Rng;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::utils::schema::users;
use super::db;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserInfo {
    pub id: Option<Uuid>,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(&conn)?;
        Ok(user)
    }

    pub fn find_by_email(eml: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table
            .filter(users::email.eq(eml))
            .first(&conn)?;
        Ok(user)
    }

    pub fn create(info: UserInfo) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let mut user = User::from(info);
        user.hash_password()?;

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(info: UserInfo) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        if let Some(id) = info.id {
            let user = diesel::update(users::table)
                .filter(users::id.eq(id))
                .set(info)
                .get_result(&conn)?;

            Ok(user)
        } else {
            Err(ApiError::new(400, "User id must provide".to_owned()))
        }
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            users::table
                .filter(users::id.eq(id))
        )
            .execute(&conn)?;

        Ok(res)
    }

    /// hash password
    fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|err| {
                ApiError::new(500, format!("Failed to hash password: {}", err))
            })?;

        Ok(())
    }

    pub fn verify_password(hash: &str, password: &str) -> Result<bool, ApiError> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|e| {
                ApiError::new(500, format!("Failed to verfify password: {}", e))
            })
    }

}

impl From<UserInfo> for User {
    fn from(info: UserInfo) -> Self {
        User {
            id: match info.id {
                Some(id) => id,
                None => Uuid::new_v4(),
            },
            email: info.email,
            name: info.name,
            password: info.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
