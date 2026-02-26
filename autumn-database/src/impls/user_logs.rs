use anyhow::Context as _;

use crate::database::Database;

#[derive(Clone, Debug)]
pub struct MessageSnapshot {
    pub channel_id: u64,
    pub message_id: u64,
    pub author_user_id: u64,
    pub content: String,
    pub attachment_summary: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NewMessageSnapshot<'a> {
    pub guild_id: u64,
    pub channel_id: u64,
    pub message_id: u64,
    pub author_user_id: u64,
    pub content: &'a str,
    pub attachment_summary: Option<&'a str>,
    pub updated_at: u64,
}

#[derive(Clone, Debug)]
pub struct NewUserLog<'a> {
    pub guild_id: u64,
    pub channel_id: u64,
    pub message_id: Option<u64>,
    pub author_user_id: Option<u64>,
    pub event_type: &'a str,
    pub before_content: Option<&'a str>,
    pub after_content: Option<&'a str>,
    pub attachment_summary: Option<&'a str>,
    pub created_at: u64,
}

#[derive(Clone, Debug)]
pub struct UserLogEntry {
    pub channel_id: u64,
    pub message_id: Option<u64>,
    pub author_user_id: Option<u64>,
    pub event_type: String,
    pub before_content: Option<String>,
    pub after_content: Option<String>,
    pub attachment_summary: Option<String>,
    pub created_at: u64,
}

pub struct UserLogFilters<'a> {
    pub author_user_id: Option<u64>,
    pub event_type: Option<&'a str>,
    pub limit: u32,
}

#[derive(sqlx::FromRow)]
struct SnapshotRow {
    channel_id: i64,
    message_id: i64,
    author_user_id: i64,
    content: String,
    attachment_summary: Option<String>,
}

#[derive(sqlx::FromRow)]
struct UserLogRow {
    channel_id: i64,
    message_id: Option<i64>,
    author_user_id: Option<i64>,
    event_type: String,
    before_content: Option<String>,
    after_content: Option<String>,
    attachment_summary: Option<String>,
    created_at: i64,
}

pub async fn upsert_message_snapshot(
    db: &Database,
    snapshot: NewMessageSnapshot<'_>,
) -> anyhow::Result<()> {
    let guild_id_i64 = i64::try_from(snapshot.guild_id).context("guild_id out of i64 range")?;
    let channel_id_i64 =
        i64::try_from(snapshot.channel_id).context("channel_id out of i64 range")?;
    let message_id_i64 =
        i64::try_from(snapshot.message_id).context("message_id out of i64 range")?;
    let author_user_id_i64 =
        i64::try_from(snapshot.author_user_id).context("author_user_id out of i64 range")?;
    let updated_at_i64 =
        i64::try_from(snapshot.updated_at).context("updated_at out of i64 range")?;

    sqlx::query(
        "INSERT INTO message_snapshots (
            guild_id,
            channel_id,
            message_id,
            author_user_id,
            content,
            attachment_summary,
            updated_at
         ) VALUES ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (guild_id, channel_id, message_id)
         DO UPDATE SET
            author_user_id = EXCLUDED.author_user_id,
            content = EXCLUDED.content,
            attachment_summary = EXCLUDED.attachment_summary,
            updated_at = EXCLUDED.updated_at",
    )
    .bind(guild_id_i64)
    .bind(channel_id_i64)
    .bind(message_id_i64)
    .bind(author_user_id_i64)
    .bind(snapshot.content)
    .bind(snapshot.attachment_summary)
    .bind(updated_at_i64)
    .execute(db.pool())
    .await?;

    Ok(())
}

pub async fn get_message_snapshot(
    db: &Database,
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
) -> anyhow::Result<Option<MessageSnapshot>> {
    let guild_id_i64 = i64::try_from(guild_id).context("guild_id out of i64 range")?;
    let channel_id_i64 = i64::try_from(channel_id).context("channel_id out of i64 range")?;
    let message_id_i64 = i64::try_from(message_id).context("message_id out of i64 range")?;

    let row: Option<SnapshotRow> = sqlx::query_as(
        "SELECT channel_id, message_id, author_user_id, content, attachment_summary
         FROM message_snapshots
         WHERE guild_id = $1 AND channel_id = $2 AND message_id = $3",
    )
    .bind(guild_id_i64)
    .bind(channel_id_i64)
    .bind(message_id_i64)
    .fetch_optional(db.pool())
    .await?;

    row.map(|entry| {
        Ok(MessageSnapshot {
            channel_id: u64::try_from(entry.channel_id)
                .context("channel_id row out of u64 range")?,
            message_id: u64::try_from(entry.message_id)
                .context("message_id row out of u64 range")?,
            author_user_id: u64::try_from(entry.author_user_id)
                .context("author_user_id row out of u64 range")?,
            content: entry.content,
            attachment_summary: entry.attachment_summary,
        })
    })
    .transpose()
}

pub async fn delete_message_snapshot(
    db: &Database,
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
) -> anyhow::Result<()> {
    let guild_id_i64 = i64::try_from(guild_id).context("guild_id out of i64 range")?;
    let channel_id_i64 = i64::try_from(channel_id).context("channel_id out of i64 range")?;
    let message_id_i64 = i64::try_from(message_id).context("message_id out of i64 range")?;

    sqlx::query(
        "DELETE FROM message_snapshots
         WHERE guild_id = $1 AND channel_id = $2 AND message_id = $3",
    )
    .bind(guild_id_i64)
    .bind(channel_id_i64)
    .bind(message_id_i64)
    .execute(db.pool())
    .await?;

    Ok(())
}

pub async fn insert_user_log(db: &Database, entry: NewUserLog<'_>) -> anyhow::Result<()> {
    let guild_id_i64 = i64::try_from(entry.guild_id).context("guild_id out of i64 range")?;
    let channel_id_i64 = i64::try_from(entry.channel_id).context("channel_id out of i64 range")?;
    let message_id_i64 = entry
        .message_id
        .map(i64::try_from)
        .transpose()
        .context("message_id out of i64 range")?;
    let author_user_id_i64 = entry
        .author_user_id
        .map(i64::try_from)
        .transpose()
        .context("author_user_id out of i64 range")?;
    let created_at_i64 = i64::try_from(entry.created_at).context("created_at out of i64 range")?;

    sqlx::query(
        "INSERT INTO user_logs (
            guild_id,
            channel_id,
            message_id,
            author_user_id,
            event_type,
            before_content,
            after_content,
            attachment_summary,
            created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(guild_id_i64)
    .bind(channel_id_i64)
    .bind(message_id_i64)
    .bind(author_user_id_i64)
    .bind(entry.event_type)
    .bind(entry.before_content)
    .bind(entry.after_content)
    .bind(entry.attachment_summary)
    .bind(created_at_i64)
    .execute(db.pool())
    .await?;

    Ok(())
}

pub async fn list_recent_user_logs(
    db: &Database,
    guild_id: u64,
    filters: UserLogFilters<'_>,
) -> anyhow::Result<Vec<UserLogEntry>> {
    let guild_id_i64 = i64::try_from(guild_id).context("guild_id out of i64 range")?;
    let author_user_id_i64 = filters
        .author_user_id
        .map(i64::try_from)
        .transpose()
        .context("author_user_id out of i64 range")?;
    let limit_i64 = i64::from(filters.limit.clamp(1, 200));

    let rows: Vec<UserLogRow> = sqlx::query_as(
        "SELECT
            channel_id,
            message_id,
            author_user_id,
            event_type,
            before_content,
            after_content,
            attachment_summary,
            created_at
         FROM user_logs
         WHERE guild_id = $1
           AND ($2::BIGINT IS NULL OR author_user_id = $2)
           AND ($3::TEXT IS NULL OR LOWER(event_type) = LOWER($3))
         ORDER BY created_at DESC
         LIMIT $4",
    )
    .bind(guild_id_i64)
    .bind(author_user_id_i64)
    .bind(filters.event_type)
    .bind(limit_i64)
    .fetch_all(db.pool())
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        out.push(UserLogEntry {
            channel_id: u64::try_from(row.channel_id).context("channel_id row out of u64 range")?,
            message_id: row
                .message_id
                .map(u64::try_from)
                .transpose()
                .context("message_id row out of u64 range")?,
            author_user_id: row
                .author_user_id
                .map(u64::try_from)
                .transpose()
                .context("author_user_id row out of u64 range")?,
            event_type: row.event_type,
            before_content: row.before_content,
            after_content: row.after_content,
            attachment_summary: row.attachment_summary,
            created_at: u64::try_from(row.created_at).context("created_at row out of u64 range")?,
        });
    }

    Ok(out)
}
