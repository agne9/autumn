DROP TABLE IF EXISTS guild_mod_config;

DROP INDEX IF EXISTS mod_cases_guild_case_code_number_idx;

ALTER TABLE mod_cases
    DROP COLUMN IF EXISTS action_case_number;

ALTER TABLE mod_cases
    DROP COLUMN IF EXISTS case_code;
