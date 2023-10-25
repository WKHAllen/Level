use crate::{convert_file_name, TABLES};
use backend_common::*;
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection, SqliteLockingMode};
use sqlx::{ConnectOptions, Connection};
use std::error::Error as StdError;
use std::future::Future;
use std::io;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;
use tokio::fs::{self, File, OpenOptions};

/// The directory in which temporary database files are stored.
pub(crate) const TEMP_DB_DIR: &str = "temp";

/// The file extension used to identify database files.
pub(crate) const DB_EXT: &str = "db";

/// Creates the temporary database directory if it does not already exist.
pub(crate) async fn init_temp_db_dir() -> io::Result<()> {
    let root_path = project_root::get_project_root().unwrap();
    let temp_db_dir = root_path.join(TEMP_DB_DIR);

    if !temp_db_dir.exists() {
        fs::create_dir(&temp_db_dir).await?;
    }

    Ok(())
}

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

/// The underlying database connection implementation.
pub type DBImpl = SqliteConnection;

/// A representation of a database.
#[derive(Debug)]
pub struct DB {
    /// The internal database connection.
    conn: DBImpl,
    /// The name of the database.
    name: String,
}

impl DB {
    /// Checks if a database file exists.
    pub fn exists(name: &str) -> bool {
        let db_path = get_db_path(name);
        Path::new(&db_path).exists()
    }

    /// Opens a database file and starts a connection.
    pub async fn open(name: &str) -> Result<Self> {
        init_temp_db_dir().await?;

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
        init_temp_db_dir().await?;

        {
            let db_path = get_db_path(name);
            File::create(&db_path).await?;
        }

        let mut this = Self::open(name).await?;
        this.init_tables().await?;

        Ok(this)
    }

    /// Creates a new database file, giving access to the raw file via a closure for reading and writing before starting the connection.
    pub async fn create_with<F, T, E>(name: &str, f: F) -> Result<Self, E>
    where
        F: FnOnce(File) -> T,
        T: Future<Output = StdResult<(), E>>,
        E: StdError + Send + Sync + 'static,
    {
        init_temp_db_dir().await?;

        {
            let db_path = get_db_path(name);
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(db_path)
                .await?;

            if let Err(err) = f(file).await {
                return Err(Error::Other(err));
            }
        }

        match Self::open(name).await {
            Ok(value) => Ok(value),
            Err(err) => Err(match err {
                Error::Expected(err) => Error::Expected(err),
                Error::Unexpected(err) => Error::Unexpected(err),
                Error::Other(_) => unreachable!(),
            }),
        }
    }

    /// Opens a database file, creating it if it doesn't exist.
    pub async fn open_or_create(name: &str) -> Result<Self> {
        if Self::exists(name) {
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

    /// Performs a series of operations within a database transaction,
    /// committing if successful or rolling back if not.
    pub async fn transaction<F, T, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut DBImpl) -> T + Send + Sync + 'static,
        T: Future<Output = Result<R>> + Send,
        R: Send,
    {
        self.conn
            .transaction(|conn| Box::pin(async move { f(conn).await }))
            .await
    }

    /// Pauses the connection temporarily, giving access to the raw file via a
    /// closure for reading and writing before continuing.
    pub async fn pause_with<F, T, E>(&mut self, f: F) -> Result<(), E>
    where
        F: FnOnce(File) -> T,
        T: Future<Output = StdResult<(), E>>,
        E: StdError + Send + Sync + 'static,
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

            if let Err(err) = f(file).await {
                return Err(Error::Other(err));
            }
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
    pub async fn delete_with<F, T, E>(self, f: F) -> Result<(), E>
    where
        F: FnOnce(File) -> T,
        T: Future<Output = StdResult<(), E>>,
        E: StdError + Send + Sync + 'static,
    {
        self.conn.close().await?;

        let db_path = get_db_path(&self.name);

        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&db_path)
                .await?;

            if let Err(err) = f(file).await {
                return Err(Error::Other(err));
            }
        }

        fs::remove_file(&db_path).await?;

        Ok(())
    }
}

impl Deref for DB {
    type Target = DBImpl;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for DB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}
