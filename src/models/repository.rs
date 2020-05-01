use diesel::prelude::*;
use uuid::Uuid;

use super::db;
use crate::errors::ApiError;
use crate::utils::schema::repositories;

/// Repository models defines here used for
/// database
#[derive(Serialize, Deserialize, Insertable, Queryable, Clone)]
#[table_name = "repositories"]
pub struct Repository {
    pub id: i32,
    pub belong_to: Option<Uuid>,
    pub repo_name: String,
    pub is_public: bool,
    pub is_valid: bool,
}

#[derive(Deserialize)]
pub struct DeleteInfo {
    pub repo_name: String,
}

#[derive(Deserialize)]
pub struct PageInfo {
    pub repo_name: String,
    pub page: i32,
}

impl Repository {
    pub fn create(belong: Option<&Uuid>, repo_name: &str, is_public: bool) -> Result<Repository, ApiError> {
        let conn = db::connection()?;

        let result = diesel::insert_into(repositories::table)
            .values(&(
                repositories::belong_to.eq(belong),
                repositories::repo_name.eq(repo_name),
                repositories::is_public.eq(is_public),
            ))
            .get_result(&conn)?;
        Ok(result)
    }

    pub fn delete(name: &str) -> Result<(), ApiError> {
        let conn = db::connection()?;

        diesel::update(repositories::table
                       .filter(repositories::repo_name.eq(name)))
            .set(repositories::is_valid.eq(false))
            .execute(&conn)?;
        Ok(())
    }

    pub fn is_deleted(name: &str) -> Result<bool, ApiError> {
        let conn = db::connection()?;

        let result: Repository = repositories::table
            .filter(repositories::repo_name.eq(name))
            .get_result(&conn)?;
        Ok(!result.is_valid)
    }
}
