use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use std::ops::Deref;
use std::path::Path;
use tokio::fs;

/// The directory in which temporary database files are stored.
const TEMP_DB_DIR: &str = "temp";

/// Gets the path to a database file.
fn get_db_path(name: &str) -> String {
    let root_path = project_root::get_project_root().unwrap();
    let db_path = format!("{}/{}/{}.db", root_path.display(), TEMP_DB_DIR, name);
    db_path
}

/// Gets the connection string to access a database file.
fn format_conn_str(name: &str) -> String {
    let db_path = get_db_path(name);
    let conn_str = format!("sqlite:{}", db_path);
    conn_str
}

/// A representation of a database.
pub struct DB {
    /// The internal database pool.
    pool: SqlitePool,
    /// The name of the database.
    name: String,
}

impl DB {
    /// Checks if a database file exists.
    pub async fn exists(name: &str) -> bool {
        let db_path = get_db_path(name);
        Path::new(&db_path).exists()
    }

    /// Opens a database file and starts a connection pool.
    pub async fn open(name: &str) -> Result<Self> {
        let conn_str = format_conn_str(name);
        let pool = SqlitePool::connect(&conn_str).await?;

        Ok(Self {
            pool,
            name: name.to_owned(),
        })
    }

    /// Creates a new database file and starts a connection pool.
    pub async fn create(name: &str) -> Result<Self> {
        {
            let db_path = get_db_path(name);
            fs::File::create(&db_path).await?;
        }

        Self::open(name).await
    }

    /// Opens a database file, creating it if it doesn't exist.
    pub async fn open_or_create(name: &str) -> Result<Self> {
        if Self::exists(name).await {
            Self::open(name).await
        } else {
            Self::create(name).await
        }
    }

    /// Deletes the existing database file.
    pub async fn delete(self) -> Result<()> {
        self.pool.close().await;

        let db_path = get_db_path(&self.name);
        fs::remove_file(&db_path).await?;

        Ok(())
    }
}

impl Deref for DB {
    type Target = SqlitePool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
