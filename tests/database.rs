use sqlx::{PgPool, Row};

#[sqlx::test]
async fn test_database_connection(pool: PgPool) -> anyhow::Result<()> {
    let row = sqlx::query("SELECT 1 + 1 AS sum").fetch_one(&pool).await?;

    let sum: i32 = row.get("sum");

    assert_eq!(sum, 2);
    Ok(())
}
