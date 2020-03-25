use diesel::prelude::*;
use uuid::Uuid;

use super::db;
use super::user::{ClusterRole, User};
use crate::errors::ApiError;
use crate::utils::schema::{departments, users};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "departments"]
pub struct Department {
    pub id: i32,
    pub name: String,
    pub admin: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct DepartInfo {
    pub id: i32,
    pub name: String,
    pub admin: String,
    pub email: String,
}

impl DepartInfo {
    fn new(id: i32, name: String, admin: String, email: String) -> DepartInfo {
        DepartInfo {
            id,
            name,
            admin,
            email,
        }
    }
}

impl Department {
    pub fn create<T>(name: T, email: Option<T>) -> Result<Department, ApiError>
    where
        T: Into<String>,
    {
        let conn = db::connection()?;
        if let Some(eml) = email {
            let eml = eml.into();
            let user = User::find_by_email(&eml).map_err(|e| {
                if e.status_code == 404 {
                    ApiError::new(404, format!("User with email {} do not exists", &eml))
                } else {
                    e
                }
            })?;

            let info = diesel::insert_into(departments::table)
                .values(&(
                    departments::name.eq(name.into()),
                    departments::admin.eq(user.id),
                ))
                .get_result(&conn)?;
            Ok(info)
        } else {
            let info = diesel::insert_into(departments::table)
                .values(departments::name.eq(name.into()))
                .get_result(&conn)?;
            Ok(info)
        }
    }

    pub fn set_admin(id: i32, admin: &Uuid) -> Result<Department, ApiError> {
        let conn = db::connection()?;

        let info = diesel::update(departments::table.filter(departments::id.eq(id)))
            .set(departments::admin.eq(admin))
            .get_result(&conn)?;
        Ok(info)
    }

    pub fn list_all() -> Result<Vec<Department>, ApiError> {
        let conn = db::connection()?;

        let result: Vec<Department> = departments::table.get_results(&conn)?;
        Ok(result)
    }

    pub fn list_infos() -> Result<Vec<DepartInfo>, ApiError> {
        let conn = db::connection()?;

        let admins: Vec<User> = users::table
            .filter(users::role.eq(ClusterRole::DepartmentAdmin))
            .get_results(&conn)?;
        let departs: Vec<Department> = departments::table.get_results(&conn)?;
        let mut results: Vec<DepartInfo> = Vec::new();
        for depart in departs.iter() {
            if let None = depart.admin {
                results.push(DepartInfo::new(
                    depart.id,
                    depart.name.clone(),
                    "N/A".to_owned(),
                    "N/A".to_owned(),
                ))
            }
            for admin in admins.iter() {
                if depart.id == admin.belong_to.unwrap() {
                    results.push(DepartInfo::new(
                        depart.id,
                        depart.name.clone(),
                        admin.name.clone(),
                        admin.email.clone(),
                    ))
                }
            }
        }
        /*
        let results = departments::table
            .left_join(users::table.on(
                users::table.filter(users::role.eq(ClusterRole::DepartmentAdmin))
                departments::id.eq(users::belong_to).and(
                    users::role.eq(ClusterRole::DepartmentAdmin))
            ))
            .select((departments::id, departments::name, users::name, users::email))
            .load(&conn)?;*/
        Ok(results)
    }
}
