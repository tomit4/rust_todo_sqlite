use crate::SqlitePool;
pub async fn add_todo(pool: &SqlitePool, title: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id  = sqlx::query!(
        r#"
        INSERT INTO todo ( title, completed )
        VALUES ( ?1, ?2 )
        "#,
        title,
        false
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

