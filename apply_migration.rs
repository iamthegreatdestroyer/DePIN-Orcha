use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the migration SQL
    let sql = fs::read_to_string("migrations/20260115000001_create_api_keys.sql")?;

    // Connect to the database
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:depin_orcha.db").await?;

    // Execute the migration
    sqlx::raw_sql(&sql).execute(&pool).await?;

    println!("✅ Migration applied successfully!");

    pool.close().await;
    Ok(())
}
