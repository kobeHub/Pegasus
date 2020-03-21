use diesel::prelude::*;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::utils::schema::departments;
use super::db;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "departments"]
pub struct Department {
    pub id: i32,
    pub name: String,
    pub admin: Option<Uuid>,
}

impl Department {
    pub fn create<T>(name: T) -> Result<Department, ApiError>
    where
        T: Into<String>
    {
        let conn = db::connection()?;

        let info = diesel::insert_into(departments::table)
            .values(departments::name.eq(name.into()))
            .get_result(&conn)?;
        Ok(info)
    }

    pub fn set_admin(&self) -> Result<Department, ApiError> {
        let conn = db::connection()?;

        let info = diesel::update(departments::table.filter(
            departments::id.eq(self.id)))
            .set(departments::admin.eq(self.admin.unwrap()))
            .get_result(&conn)?;
        Ok(info)
    }
}
