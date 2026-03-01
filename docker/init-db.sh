#!/usr/bin/env bash
# init-db.sh — Postgres initialisation script
#
# Runs once on first database creation (docker-entrypoint-initdb.d).
# Creates two application roles with separate privilege levels:
#
#   autumn_migrator — owns schema objects, may run DDL (ALTER / CREATE / DROP)
#   autumn_app      — DML only (SELECT / INSERT / UPDATE / DELETE); no DDL
#
# Passwords are injected via environment variables from docker-compose.yml:
#   POSTGRES_MIGRATOR_PASSWORD
#   POSTGRES_APP_PASSWORD

set -euo pipefail

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL

    --------------------------------------------------------------------
    -- Migration role: full DDL privileges, owns the public schema
    --------------------------------------------------------------------
    CREATE ROLE autumn_migrator
        WITH LOGIN PASSWORD '${POSTGRES_MIGRATOR_PASSWORD}';

    GRANT ALL PRIVILEGES ON DATABASE "${POSTGRES_DB}" TO autumn_migrator;

    -- The migrator owns the public schema so it can CREATE tables, indexes, etc.
    ALTER SCHEMA public OWNER TO autumn_migrator;
    GRANT ALL ON SCHEMA public TO autumn_migrator;

    --------------------------------------------------------------------
    -- Application role: DML only, connects with minimal permissions
    --------------------------------------------------------------------
    CREATE ROLE autumn_app
        WITH LOGIN PASSWORD '${POSTGRES_APP_PASSWORD}';

    GRANT CONNECT ON DATABASE "${POSTGRES_DB}" TO autumn_app;
    GRANT USAGE ON SCHEMA public TO autumn_app;

    -- Every table/sequence created by autumn_migrator going forward
    -- is automatically readable/writable by autumn_app.
    ALTER DEFAULT PRIVILEGES FOR ROLE autumn_migrator IN SCHEMA public
        GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO autumn_app;

    ALTER DEFAULT PRIVILEGES FOR ROLE autumn_migrator IN SCHEMA public
        GRANT USAGE, SELECT ON SEQUENCES TO autumn_app;

EOSQL
