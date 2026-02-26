use anyhow::Context as _;

use crate::database::Database;

pub async fn get_userlog_channel_id(db: &Database, guild_id: u64) -> anyhow::Result<Option<u64>> {
    let guild_id_i64 = i64::try_from(guild_id).context("guild_id out of i64 range")?;

    let channel_id: Option<i64> = sqlx::query_scalar(
        "SELECT userlog_channel_id FROM guild_userlog_config WHERE guild_id = $1",
    )
    .bind(guild_id_i64)
    .fetch_optional(db.pool())
    .await?
    .flatten();

    channel_id
        .map(u64::try_from)
        .transpose()
        .context("userlog_channel_id out of u64 range")
}

pub async fn set_userlog_channel_id(
    db: &Database,
    guild_id: u64,
    channel_id: u64,
) -> anyhow::Result<()> {
    let guild_id_i64 = i64::try_from(guild_id).context("guild_id out of i64 range")?;
    let channel_id_i64 = i64::try_from(channel_id).context("channel_id out of i64 range")?;

    sqlx::query(
        "INSERT INTO guild_userlog_config (guild_id, userlog_channel_id)
         VALUES ($1, $2)
         ON CONFLICT (guild_id) DO UPDATE SET userlog_channel_id = EXCLUDED.userlog_channel_id",
    )
    .bind(guild_id_i64)
    .bind(channel_id_i64)
    .execute(db.pool())
    .await?;

    Ok(())
}

pub async fn clear_userlog_channel_id(db: &Database, guild_id: u64) -> anyhow::Result<()> {
    let guild_id_i64 = i64::try_from(guild_id).context("guild_id out of i64 range")?;

    sqlx::query("DELETE FROM guild_userlog_config WHERE guild_id = $1")
        .bind(guild_id_i64)
        .execute(db.pool())
        .await?;

    Ok(())
}
