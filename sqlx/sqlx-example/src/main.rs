use std::env;

use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("DATABASE_URL must be set"),
    };
    let pool = match MySqlPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => panic!("Error: {}", e),
    };

    if let Err(e) = sqlx::query!(
        r#"
        INSERT INTO rust(
            id,
            name
        )
        VALUES(
            1,
            "halo"
        );
        "#,
    )
    .execute(&pool)
    .await
    {
        panic!("Error: {}", e);
    }
}
