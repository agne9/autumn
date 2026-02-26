CREATE TABLE IF NOT EXISTS guild_userlog_config (
    guild_id BIGINT PRIMARY KEY,
    userlog_channel_id BIGINT
);

CREATE TABLE IF NOT EXISTS message_snapshots (
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    author_user_id BIGINT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    attachment_summary TEXT,
    updated_at BIGINT NOT NULL,
    PRIMARY KEY (guild_id, channel_id, message_id)
);

CREATE TABLE IF NOT EXISTS user_logs (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    message_id BIGINT,
    author_user_id BIGINT,
    event_type TEXT NOT NULL,
    before_content TEXT,
    after_content TEXT,
    attachment_summary TEXT,
    created_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS user_logs_guild_created_idx
    ON user_logs (guild_id, created_at DESC);
