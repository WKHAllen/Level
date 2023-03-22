use anyhow::Result;
use sqlx::sqlite::SqlitePool;

/// A representation of a database.
pub struct DB {
    /// The internal database pool.
    pub(crate) pool: SqlitePool,
}

impl DB {
    /// Open the database file and start a connection pool.
    pub async fn open(name: &str) -> Result<Self> {
        let root_path = project_root::get_project_root()?;
        let conn_str = format!("sqlite:{}/assets/{}.db", root_path.display(), name);
        let pool = SqlitePool::connect(&conn_str).await?;

        Ok(Self { pool })
    }
}
