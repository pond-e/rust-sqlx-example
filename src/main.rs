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

#[derive(Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub address: Option<String>,
}

pub async fn create_user(
    pool: &sqlx::SqlitePool,
    request: CreateUserRequest,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "insert into users (name, email, address) values (?, ?, ?)",
        request.name,
        request.email,
        request.address
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().expect("Failed to load .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;

    let req1 = CreateUserRequest {
        name: "ike".to_string(),
        email: "ike2@example.com".to_string(),
        address: None,
    };
    let _new_id = create_user(&pool, req1).await?;

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
