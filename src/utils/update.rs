use crate::SqlitePool;
pub async fn update_todo(pool: &SqlitePool, id: i64, title: &str) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE todo
        SET title = ?
        WHERE id = ?
        "#,
        title,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}
