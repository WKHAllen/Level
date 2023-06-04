use crate::{convert_file_name, TABLES};
use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection, SqliteLockingMode};
use sqlx::{ConnectOptions, Connection};
use std::error::Error;
use std::future::Future;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::str::FromStr;
use tokio::fs::{self, File, OpenOptions};

/// The directory in which temporary database files are stored.
pub(crate) const TEMP_DB_DIR: &str = "temp";

/// The file extension used to identify database files.
pub(crate) const DB_EXT: &str = "db";

/// Gets the path to a database file.
pub(crate) fn get_db_path(name: &str) -> String {
    let file_name = convert_file_name(name);
    let root_path = project_root::get_project_root().unwrap();
    let db_path = format!(
        "{}/{}/{}.{}",
        root_path.display(),
        TEMP_DB_DIR,
        file_name,
        DB_EXT
    );
    db_path
}

/// Gets the path to a sql initialization file.
pub(crate) fn get_sql_init_path(table: &str) -> String {
    let root_path = project_root::get_project_root().unwrap();
    let sql_init_path = format!("{}/db/sql/init/{}.sql", root_path.display(), table);
    sql_init_path
}

/// A representation of a database.
#[derive(Debug)]
pub struct DB {
    /// The internal database connection.
    conn: SqliteConnection,
    /// The name of the database.
    name: String,
}

impl DB {
    /// Checks if a database file exists.
    pub async fn exists(name: &str) -> bool {
        let db_path = get_db_path(name);
        Path::new(&db_path).exists()
    }

    /// Opens a database file and starts a connection.
    pub async fn open(name: &str) -> Result<Self> {
        let db_path = get_db_path(name);
        let conn = SqliteConnectOptions::new()
            .filename(&db_path)
            .locking_mode(SqliteLockingMode::Exclusive)
            .connect()
            .await?;

        Ok(Self {
            conn,
            name: name.to_owned(),
        })
    }

    /// Creates a new database file and starts a connection.
    pub async fn create(name: &str) -> Result<Self> {
        {
            let db_path = get_db_path(name);
            File::create(&db_path).await?;
        }

        let mut this = Self::open(name).await?;
        this.init_tables().await?;

        Ok(this)
    }

    /// Creates a new database file, giving access to the raw file via a closure for reading and writing before starting the connection.
    pub async fn create_with<F, T, E>(name: &str, f: F) -> Result<Self>
    where
        F: FnOnce(File) -> T,
        T: Future<Output = Result<(), E>>,
        E: Error + Send + Sync + 'static,
    {
        {
            let db_path = get_db_path(name);
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(db_path)
                .await?;

            f(file).await?;
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

    /// Initialize a database table.
    async fn init_table(&mut self, table: &str) -> Result<()> {
        let sql_path = get_sql_init_path(table);
        let sql_bytes = fs::read(sql_path).await?;
        let sql_str = String::from_utf8(sql_bytes)?;

        sqlx::query(&sql_str).execute(&mut **self).await?;

        Ok(())
    }

    /// Initializes all database tables.
    async fn init_tables(&mut self) -> Result<()> {
        for table in TABLES {
            self.init_table(table).await?;
        }

        Ok(())
    }

    /// Pauses the connection temporarily, giving access to the raw file via a closure for reading and writing before continuing.
    pub async fn pause_with<F, T, E>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(File) -> T,
        T: Future<Output = Result<(), E>>,
        E: Error + Send + Sync + 'static,
    {
        let temp_conn = SqliteConnectOptions::from_str("sqlite::memory:")?
            .read_only(true)
            .connect()
            .await?;

        let conn = mem::replace(&mut self.conn, temp_conn);
        conn.close().await?;

        let db_path = get_db_path(&self.name);

        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&db_path)
                .await?;

            f(file).await?;
        }

        let conn = SqliteConnectOptions::new()
            .filename(&db_path)
            .locking_mode(SqliteLockingMode::Exclusive)
            .connect()
            .await?;

        let temp_conn = mem::replace(&mut self.conn, conn);
        temp_conn.close().await?;

        Ok(())
    }

    /// Deletes the existing database file.
    pub async fn delete(self) -> Result<()> {
        self.conn.close().await?;

        let db_path = get_db_path(&self.name);
        fs::remove_file(&db_path).await?;

        Ok(())
    }

    /// Deletes the existing database file, giving access to the raw file via a closure for reading and writing before deleting the file.
    pub async fn delete_with<F, T, E>(self, f: F) -> Result<()>
    where
        F: FnOnce(File) -> T,
        T: Future<Output = Result<(), E>>,
        E: Error + Send + Sync + 'static,
    {
        self.conn.close().await?;

        let db_path = get_db_path(&self.name);

        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&db_path)
                .await?;

            f(file).await?;
        }

        fs::remove_file(&db_path).await?;

        Ok(())
    }
}

impl Deref for DB {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for DB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}
