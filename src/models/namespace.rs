use uuid::Uuid;

use super::db;
use crate::errors::ApiError;
use crate::utils::schema::namespaces;

#[derive(Serialize, Deserialize, Insertable, Quaryable)]
#[table_name = "namspaces"]
pub struct Namespace {
    pub id: i32,
    pub uid: Uuid,
    pub namespace: String,
    pub valid: bool,
}

impl Namespace {
    fn create(uid: Uuid, ns: T) -> Result<Namespace, ApiError>
    where
        T: Into<String>
    {
        let conn = db::connection()?;

        let result = diesel::insert(namespaces::table);

    }
}
