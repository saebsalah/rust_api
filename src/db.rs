use super::constants;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Error, Pool, Sqlite,
};
use std::str::FromStr;

pub async fn establish_connection() -> Result<Pool<Sqlite>, Error> {
    let connection_options =
        SqliteConnectOptions::from_str(constants::DATABASE_URL)?.create_if_missing(true);

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(constants::POOL_MAX_CONNECTIONS)
        .connect_with(connection_options)
        .await?;

    sqlx::query(&format!(
        "
    CREATE TABLE IF NOT EXISTS {} (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      title text,
      author text
    );
    CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name text
      );
    ",
        constants::BOOKS_TABLE,
        constants::AUTHORS_TABLE
    ))
    .execute(&sqlite_pool.clone())
    .await?;

    Ok(sqlite_pool)
}
