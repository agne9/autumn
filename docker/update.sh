#!/usr/bin/env bash
# update.sh — rebuild, migrate, and restart the bot (Docker)
#
# Usage (from repo root):
#   ./docker/update.sh
#
# Pull the latest code manually first:
#   git pull
#
# With LLM profile active:
#   COMPOSE_PROFILES=llm ./docker/update.sh
#
# Using Podman? Use update-podman.sh instead.

set -euo pipefail

# Ensure SYSTEM_PROMPT.md exists so the volume mount never errors.
# Leave it empty to use the built-in default prompt.
touch "$(dirname "$0")/../SYSTEM_PROMPT.md"

echo "==> Rebuilding bot and migrator images..."
docker compose build autumn-bot migrator

echo "==> Running migrations..."
docker compose run --rm migrator

echo "==> Restarting bot..."
docker compose up -d autumn-bot

echo "==> Done. Bot is up to date."
