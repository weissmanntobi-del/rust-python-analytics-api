use crate::{
    dto::analytics::{EventCount, TimeseriesPoint},
    models::event::EventRecord,
    state::QueuedEvent,
};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn insert_event(db: &PgPool, event: &QueuedEvent) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO events (id, user_id, event_name, page_url, session_id, properties, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(event.id)
    .bind(event.user_id)
    .bind(&event.event_name)
    .bind(&event.page_url)
    .bind(&event.session_id)
    .bind(&event.properties)
    .bind(event.created_at)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn list_recent_events(
    db: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<EventRecord>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, user_id, event_name, page_url, session_id, properties, created_at
        FROM events
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| EventRecord {
            id: row.try_get("id").expect("id"),
            user_id: row.try_get("user_id").expect("user_id"),
            event_name: row.try_get("event_name").expect("event_name"),
            page_url: row.try_get("page_url").expect("page_url"),
            session_id: row.try_get("session_id").expect("session_id"),
            properties: row.try_get("properties").expect("properties"),
            created_at: row.try_get("created_at").expect("created_at"),
        })
        .collect())
}

pub async fn summary_totals(
    db: &PgPool,
    user_id: Uuid,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<(i64, i64), sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            COUNT(*)::BIGINT AS total_events,
            COUNT(DISTINCT session_id)::BIGINT AS unique_sessions
        FROM events
        WHERE user_id = $1
          AND created_at >= $2
          AND created_at <= $3
        "#,
    )
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_one(db)
    .await?;

    Ok((
        row.try_get("total_events")?,
        row.try_get("unique_sessions")?,
    ))
}

pub async fn top_events(
    db: &PgPool,
    user_id: Uuid,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<EventCount>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT event_name, COUNT(*)::BIGINT AS total
        FROM events
        WHERE user_id = $1
          AND created_at >= $2
          AND created_at <= $3
        GROUP BY event_name
        ORDER BY total DESC, event_name ASC
        LIMIT 5
        "#,
    )
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| EventCount {
            event_name: row.try_get("event_name").expect("event_name"),
            total: row.try_get("total").expect("total"),
        })
        .collect())
}

pub async fn timeseries(
    db: &PgPool,
    user_id: Uuid,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    bucket: &str,
) -> Result<Vec<TimeseriesPoint>, sqlx::Error> {
    let trunc = match bucket {
        "hour" => "hour",
        _ => "day",
    };

    let query = format!(
        r#"
        SELECT
            date_trunc('{trunc}', created_at) AS bucket_start,
            COUNT(*)::BIGINT AS total
        FROM events
        WHERE user_id = $1
          AND created_at >= $2
          AND created_at <= $3
        GROUP BY bucket_start
        ORDER BY bucket_start ASC
        "#
    );

    let rows = sqlx::query(&query)
        .bind(user_id)
        .bind(from)
        .bind(to)
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| TimeseriesPoint {
            bucket_start: row.try_get("bucket_start").expect("bucket_start"),
            total: row.try_get("total").expect("total"),
        })
        .collect())
}
