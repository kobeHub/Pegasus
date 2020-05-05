use diesel::prelude::*;
use diesel::result::Error as DBError;
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
    #[serde(rename = "repoName")]
    pub repo_name: String,
}

#[derive(Deserialize)]
pub struct PageInfo {
    pub name: String,
    pub page: i32,
}

#[derive(Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "repoName")]
    pub repo_name: String,
    pub tag: String,
}

#[derive(PartialEq)]
pub enum RepoRecordState {
    NotFound,
    Deleted,
    Active,
}

impl Repository {
    pub fn create(
        belong: Option<&Uuid>,
        repo_name: &str,
        is_public: bool,
        no_record: bool,
    ) -> Result<Repository, ApiError> {
        let conn = db::connection()?;
        if no_record {
            let result = diesel::insert_into(repositories::table)
                .values(&(
                    repositories::belong_to.eq(belong),
                    repositories::repo_name.eq(repo_name),
                    repositories::is_public.eq(is_public),
                ))
                .get_result(&conn)?;
            Ok(result)
        } else {
            let result =
                diesel::update(repositories::table.filter(repositories::repo_name.eq(repo_name)))
                    .set((
                        repositories::belong_to.eq(belong),
                        repositories::is_public.eq(is_public),
                        repositories::is_valid.eq(true),
                    ))
                    .get_result(&conn)?;
            Ok(result)
        }
    }

    pub fn delete(name: &str) -> Result<(), ApiError> {
        let conn = db::connection()?;

        diesel::update(repositories::table.filter(repositories::repo_name.eq(name)))
            .set(repositories::is_valid.eq(false))
            .execute(&conn)?;
        Ok(())
    }

    pub fn record_state(name: &str) -> Result<RepoRecordState, ApiError> {
        let conn = db::connection()?;

        let result: Result<Repository, DBError> = repositories::table
            .filter(repositories::repo_name.eq(name))
            .get_result(&conn);
        match result {
            Ok(repo) => {
                if repo.is_valid {
                    Ok(RepoRecordState::Active)
                } else {
                    Ok(RepoRecordState::Deleted)
                }
            }
            Err(e) => match e {
                DBError::NotFound => Ok(RepoRecordState::NotFound),
                _ => Err(ApiError::from(e)),
            },
        }
    }

    pub fn get_public() -> Result<Vec<String>, ApiError> {
        let conn = db::connection()?;

        let result: Vec<String> = repositories::table
            .filter(repositories::is_public.eq(true).and(repositories::is_valid.eq(true)))
            .get_results(&conn)?
            .iter()
            .map(Repository::repo_name)
            .collect();
        Ok(result)
    }

    pub fn get_repos_by_uid(id: &Uuid) -> Result<Vec<String>, ApiError> {
        let conn = db::connection()?;

        let results: Vec<String> = repositories::table
            .filter(repositories::belong_to.eq(id))
            .get_results(&conn)?
            .iter()
            .map(Repository::repo_name)
            .collect();
        Ok(results)
    }

    fn repo_name(&self) -> String {
        self.repo_name.clone()
    }
}
