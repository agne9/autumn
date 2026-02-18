ALTER TABLE mod_cases
    ADD COLUMN IF NOT EXISTS case_code TEXT NOT NULL DEFAULT 'M';

ALTER TABLE mod_cases
    ADD COLUMN IF NOT EXISTS action_case_number BIGINT NOT NULL DEFAULT 0;

WITH ranked AS (
    SELECT
        id,
        CASE
            WHEN action = 'warn' THEN 'W'
            WHEN action = 'ban' THEN 'B'
            WHEN action = 'kick' THEN 'K'
            WHEN action = 'timeout' THEN 'T'
            WHEN action = 'unban' THEN 'UB'
            WHEN action = 'untimeout' THEN 'UT'
            WHEN action = 'unwarn' THEN 'UW'
            WHEN action = 'unwarn_all' THEN 'UWA'
            WHEN action = 'purge' THEN 'P'
            WHEN action = 'terminate' THEN 'TR'
            ELSE 'M'
        END AS resolved_code,
        ROW_NUMBER() OVER (
            PARTITION BY guild_id,
            CASE
                WHEN action = 'warn' THEN 'W'
                WHEN action = 'ban' THEN 'B'
                WHEN action = 'kick' THEN 'K'
                WHEN action = 'timeout' THEN 'T'
                WHEN action = 'unban' THEN 'UB'
                WHEN action = 'untimeout' THEN 'UT'
                WHEN action = 'unwarn' THEN 'UW'
                WHEN action = 'unwarn_all' THEN 'UWA'
                WHEN action = 'purge' THEN 'P'
                WHEN action = 'terminate' THEN 'TR'
                ELSE 'M'
            END
            ORDER BY created_at ASC, id ASC
        ) AS resolved_number
    FROM mod_cases
)
UPDATE mod_cases AS mc
SET
    case_code = ranked.resolved_code,
    action_case_number = ranked.resolved_number
FROM ranked
WHERE mc.id = ranked.id;

CREATE UNIQUE INDEX IF NOT EXISTS mod_cases_guild_case_code_number_idx
    ON mod_cases (guild_id, case_code, action_case_number);

CREATE TABLE IF NOT EXISTS guild_mod_config (
    guild_id BIGINT PRIMARY KEY,
    modlog_channel_id BIGINT
);
