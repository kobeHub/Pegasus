use diesel::prelude::*;
use diesel::result::Error as DBError;

use super::db;
use super::repository::RepoRecordState;
use crate::errors::ApiError;
use crate::utils::schema::tags;

/// The `Tag` schema only record github file exists
#[derive(Deserialize, Serialize, Clone, Insertable, Queryable)]
#[table_name = "tags"]
pub struct Tag {
    id: i32,
    repo_name: String,
    tag_name: String,
    is_valid: bool,
}

pub type TagRecordState = RepoRecordState;

impl Tag {
    pub fn create(repo_name: &str, tag_name: &str) -> Result<Tag, ApiError> {
        let conn = db::connection()?;

        let result = diesel::insert_into(tags::table)
            .values(&(tags::repo_name.eq(repo_name), tags::tag_name.eq(tag_name)))
            .get_result(&conn)?;
        Ok(result)
    }

    pub fn recreate(repo_name: &str, tag_name: &str) -> Result<(), ApiError> {
        let conn = db::connection()?;

        diesel::update(
            tags::table.filter(
                tags::repo_name
                    .eq(repo_name)
                    .and(tags::tag_name.eq(tag_name)),
            ),
        )
        .set(tags::is_valid.eq(true))
        .execute(&conn)?;
        Ok(())
    }

    pub fn record_state(repo_name: &str, tag_name: &str) -> Result<TagRecordState, ApiError> {
        let conn = db::connection()?;

        let result: Result<Tag, DBError> = tags::table
            .filter(
                tags::repo_name
                    .eq(repo_name)
                    .and(tags::tag_name.eq(tag_name)),
            )
            .get_result(&conn);
        match result {
            Ok(repo) => {
                if repo.is_valid {
                    Ok(TagRecordState::Active)
                } else {
                    Ok(TagRecordState::Deleted)
                }
            }
            Err(e) => match e {
                DBError::NotFound => Ok(TagRecordState::NotFound),
                _ => Err(ApiError::from(e)),
            },
        }
    }

    pub fn delete(repo_name: &str, tag_name: &str) -> Result<(), ApiError> {
        let conn = db::connection()?;

        diesel::update(
            tags::table.filter(
                tags::repo_name
                    .eq(repo_name)
                    .and(tags::tag_name.eq(tag_name)),
            ),
        )
        .set(tags::is_valid.eq(false))
        .execute(&conn)?;
        Ok(())
    }
}
