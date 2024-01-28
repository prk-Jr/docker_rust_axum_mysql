use dotenvy::dotenv;
use sqlx::*;
use std::env;

/// .
///
/// # Panics
///
/// Panics if fails to acquire a database connection
///
/// # Errors
///
/// This function will return an error if fails to acquire a pool
pub async fn connect_to_database() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("Env var unavailable");

    println!("databse {database_url}");
    MySqlPool::connect(&database_url).await
}
