use anyhow::Result;
use db::DB;

const TEST_DB_NAME: &str = "test";

/// Initialize the test database on build.
#[tokio::main]
async fn main() -> Result<()> {
    if DB::exists(TEST_DB_NAME).await {
        DB::open(TEST_DB_NAME).await?.delete().await?;
    }

    let db = DB::create(TEST_DB_NAME).await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS quotes (
            id TEXT NOT NULL,
            quote TEXT NOT NULL
        );
        ",
    )
    .execute(&*db)
    .await?;

    Ok(())
}
