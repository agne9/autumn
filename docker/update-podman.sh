#!/usr/bin/env bash
# update-podman.sh — rebuild, migrate, and restart the bot (Podman)
#
# Usage (from repo root):
#   ./docker/update-podman.sh
#
# Pull the latest code manually first:
#   git pull
#
# With LLM profile active:
#   ./docker/update-podman.sh llm
#
# Using Docker? Use update.sh instead.

set -euo pipefail

PROFILE="${1:-}"
PROFILE_FLAG=""
if [ -n "$PROFILE" ]; then
    PROFILE_FLAG="--profile $PROFILE"
fi

# Ensure SYSTEM_PROMPT.md exists so the volume mount never errors.
# Leave it empty to use the built-in default prompt.
touch "$(dirname "$0")/../SYSTEM_PROMPT.md"

echo "==> Rebuilding bot and migrator images..."
# shellcheck disable=SC2086
podman-compose $PROFILE_FLAG build autumn-bot migrator

echo "==> Running migrations..."
# shellcheck disable=SC2086
podman-compose $PROFILE_FLAG run --rm migrator

echo "==> Restarting bot..."
# shellcheck disable=SC2086
podman-compose $PROFILE_FLAG up -d autumn-bot

echo "==> Done. Bot is up to date."
