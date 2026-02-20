CREATE TABLE IF NOT EXISTS llm_chat_history (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS llm_chat_history_guild_channel_created_idx
    ON llm_chat_history (guild_id, channel_id, created_at DESC);