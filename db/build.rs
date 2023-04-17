use anyhow::Result;
use sqlx::{Connection, SqliteConnection};
use tokio::fs::{self, File};

/// The database tables, in order.
const TABLES: &[&str] = &[
    "account",
    "reminder",
    "budget",
    "category",
    "subcategory",
    "account_transaction",
    "tag",
    "account_transaction_tag",
    "report_template",
];

/// Initialize a database table.
async fn init_table(conn: &mut SqliteConnection, table: &str) -> Result<()> {
    let sql_path = format!("sql/init/{}.sql", table);
    let sql_bytes = fs::read(sql_path).await?;
    let sql_str = String::from_utf8(sql_bytes)?;

    sqlx::query(&sql_str).execute(conn).await?;

    Ok(())
}

/// Initialize the test database on build.
#[tokio::main]
async fn main() -> Result<()> {
    let mut test_db_path = project_root::get_project_root()?;
    test_db_path.push("temp/test_db.db");

    if test_db_path.exists() {
        fs::remove_file(&test_db_path).await?;
    }

    {
        File::create(&test_db_path).await?;
    }

    let conn_str = format!("sqlite:{}", test_db_path.display());
    let mut conn = SqliteConnection::connect(&conn_str).await?;

    for table in TABLES {
        init_table(&mut conn, table).await?;
    }

    Ok(())
}
