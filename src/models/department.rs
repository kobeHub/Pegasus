use diesel::prelude::*;

use crate::errors::ApiError;
use crate::utils::schema::departments;
use super::db;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "departments"]
pub struct Department {
    pub id: i32,
    pub name: String,
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
}
