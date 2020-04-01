use diesel::prelude::*;
use uuid::Uuid;

use super::db;
use crate::errors::ApiError;
use crate::utils::schema::namespaces;

#[derive(Serialize, Deserialize, Insertable, Queryable, Clone)]
#[table_name = "namespaces"]
pub struct Namespace {
    pub id: i32,
    pub uid: Uuid,
    pub namespace: String,
    pub valid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceInfo {
    pub uid: Uuid,
    pub ns:  String,
}

impl Namespace {
    pub fn create(info: NamespaceInfo) -> Result<Namespace, ApiError> {
        let conn = db::connection()?;

        let result = diesel::insert_into(namespaces::table)
            .values(&(
                namespaces::uid.eq(info.uid),
                namespaces::namespace.eq(info.ns),
                namespaces::valid.eq(true),
            ))
            .get_result(&conn)?;

        Ok(result)
     }

     pub fn delete(id: i32) -> Result<String, ApiError> {
         let conn = db::connection()?;

         let result: Namespace = diesel::update(namespaces::table.filter(
             namespaces::id.eq(id)))
             .set(namespaces::valid.eq(false))
             .get_result(&conn)?;
         Ok(result.namespace)
     }
}
