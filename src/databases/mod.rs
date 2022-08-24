use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_tasks")]
pub struct Tasks(sqlx::SqlitePool);