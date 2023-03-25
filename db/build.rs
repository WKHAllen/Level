use anyhow::Result;
use sqlx::{Connection, SqliteConnection};
use tokio::fs;

/// Initialize the test database on build.
#[tokio::main]
async fn main() -> Result<()> {
    let mut test_db_path = project_root::get_project_root()?;
    test_db_path.push("saves/test.db");

    if test_db_path.exists() {
        fs::remove_file(&test_db_path).await?;
    }

    {
        fs::File::create(&test_db_path).await?;
    }

    let conn_str = format!("sqlite:{}", test_db_path.display());
    let mut conn = SqliteConnection::connect(&conn_str).await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS quotes (
            id TEXT NOT NULL,
            quote TEXT NOT NULL
        );
        ",
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}
