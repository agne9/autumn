# ─────────────────────────────────────────────────────────────────────────────
# Stage 1 · builder
#   Compiles the autumn-bot release binary for the target architecture.
# ─────────────────────────────────────────────────────────────────────────────
FROM rust:1-bookworm AS builder

WORKDIR /workspace

# Copy the full workspace (migration sources must be present for sqlx::migrate!
# to embed them at compile time).
COPY . .

RUN cargo build --release -p autumn-bot --locked


# ─────────────────────────────────────────────────────────────────────────────
# Stage 2 · sqlx-installer
#   Builds sqlx-cli in a dedicated layer so it can be shared by the migrator
#   without polluting the bot runtime image.
# ─────────────────────────────────────────────────────────────────────────────
FROM rust:1-bookworm AS sqlx-installer

RUN cargo install sqlx-cli \
        --no-default-features \
        --features postgres,rustls


# ─────────────────────────────────────────────────────────────────────────────
# Stage 3 · runtime  (default target)
#   Minimal Debian slim image — runs the bot as an unprivileged user.
#   Compatible with rootless Podman (no CAP_* differences needed).
# ─────────────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# ca-certificates   — outbound TLS (Discord API, media fetching)
# libssl3           — defensive: some transitive dep may link OpenSSL at runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Fixed UID/GID so volume ownership is predictable across rootless remap
RUN groupadd --gid 10001 autumn \
    && useradd --uid 10001 --gid 10001 --no-create-home autumn

WORKDIR /app

COPY --from=builder /workspace/target/release/autumn-bot /app/autumn-bot

# autumn-bot reads SYSTEM_PROMPT.md from CWD if present; absence is handled
# gracefully. To use a custom prompt, mount it read-only at runtime:
#   -v ./SYSTEM_PROMPT.md:/app/SYSTEM_PROMPT.md:ro

USER autumn

ENTRYPOINT ["/app/autumn-bot"]


# ─────────────────────────────────────────────────────────────────────────────
# Stage 4 · migrator
#   One-shot image that runs `sqlx migrate run` against the Postgres service.
#   Run manually before first start and after every upgrade:
#
#     docker compose run --rm migrator
#     podman compose run --rm migrator
#
#   Requires DATABASE_URL to be set in the environment pointing at the
#   autumn_migrator role (DDL-privileged user).
# ─────────────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS migrator

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=sqlx-installer /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

COPY autumn-database/migrations /migrations

CMD ["sqlx", "migrate", "run", "--source", "/migrations"]
