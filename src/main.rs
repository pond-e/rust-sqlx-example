use dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use sqlx::types::chrono;

#[derive(Debug)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().expect("Failed to load .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;
    let users = sqlx::query_as!(
        User,
        "select * from users"
    )
    .fetch_all(&pool)
    .await?;
    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
