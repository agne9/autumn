-- Word filter configuration per guild
CREATE TABLE IF NOT EXISTS word_filter_config (
    guild_id   BIGINT PRIMARY KEY,
    enabled    BOOLEAN NOT NULL DEFAULT FALSE,
    action     TEXT    NOT NULL DEFAULT 'log_only'
    -- action values: 'log_only', 'delete_and_log', 'timeout_delete_and_log'
);

-- Filtered words per guild (preset or custom)
CREATE TABLE IF NOT EXISTS word_filter_words (
    id         BIGSERIAL PRIMARY KEY,
    guild_id   BIGINT  NOT NULL,
    word       TEXT    NOT NULL,
    is_preset  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at BIGINT  NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW())::BIGINT),
    UNIQUE (guild_id, word)
);

CREATE INDEX IF NOT EXISTS idx_word_filter_words_guild ON word_filter_words (guild_id);
