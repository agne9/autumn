CREATE TABLE IF NOT EXISTS mod_cases (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    case_number BIGINT NOT NULL,
    target_user_id BIGINT,
    moderator_user_id BIGINT NOT NULL,
    action TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL,
    duration_seconds BIGINT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    UNIQUE (guild_id, case_number)
);

CREATE INDEX IF NOT EXISTS mod_cases_guild_case_number_idx
    ON mod_cases (guild_id, case_number DESC);

CREATE INDEX IF NOT EXISTS mod_cases_guild_target_created_idx
    ON mod_cases (guild_id, target_user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS mod_cases_guild_moderator_created_idx
    ON mod_cases (guild_id, moderator_user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS mod_case_events (
    id BIGSERIAL PRIMARY KEY,
    case_id BIGINT NOT NULL REFERENCES mod_cases(id) ON DELETE CASCADE,
    guild_id BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    actor_user_id BIGINT NOT NULL,
    old_reason TEXT,
    new_reason TEXT,
    note TEXT,
    created_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS mod_case_events_guild_case_created_idx
    ON mod_case_events (guild_id, case_id, created_at DESC);

CREATE TABLE IF NOT EXISTS user_notes (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    target_user_id BIGINT NOT NULL,
    author_user_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    deleted_at BIGINT
);

CREATE INDEX IF NOT EXISTS user_notes_guild_target_created_idx
    ON user_notes (guild_id, target_user_id, created_at DESC);
