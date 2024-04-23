use crate::SqlitePool;
pub async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
        SELECT id, title, completed
        FROM todo
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.completed { "✔" } else { " " },
            rec.id,
            &rec.title,
        );
    }

    Ok(())
}
