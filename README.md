# Autumn

A general-purpose Discord moderation bot written in Rust, Serenity, and Poise.

Made for fun, private use, and open-source exploration.

## Features
- **Moderation**: Ban, kick, timeout, and warn users (`!ban`, `!kick`, `!timeout`, `!warn`)
- **Case Management**: Track and manage moderation cases and user notes (`!case`, `!notes`)
- **Message Purging**: Bulk delete messages with various filters (`!purge`)
- **Modlogs**: Configure and log moderation actions to a specific channel (`!modlogchannel`)
- **Utilities**: Helpful commands like `!ping`, `!help`, and `!usage`
- **Optional LLM Chat Integration**: AI-powered chat capabilities using Ollama

All commands are supported as prefix commands as well as slash commands.

With the `!help` command, the bot will provide a list of all available commands and explain how to use them.

## Internals

### Bot
- **Discord API**: [serenity](https://github.com/serenity-rs/serenity) & [poise](https://github.com/serenity-rs/poise)
- **Database**: [sqlx](https://github.com/launchbadge/sqlx) (PostgreSQL)
- **Cache**: [deadpool-redis](https://github.com/bikeshedder/deadpool) (Redis)
- **AI/LLM**: [ollama-rs](https://github.com/pepperquack/ollama-rs) & [Ollama](https://ollama.com/)

### Website
- **Framework**: [React](https://react.dev/) & [Vite](https://vitejs.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Animations**: [GSAP](https://gsap.com/)
- **Icons**: [Lucide React](https://lucide.dev/)

## Setup

While Autumn is primarily designed for private use, you are welcome to self-host it if you'd like to explore the codebase or run your own instance.

Rust must be installed, and additionally, PostgreSQL must be installed and running. If you plan to use the LLM chat features, you will also need to download and install [Ollama](https://ollama.com/).

Create a new file named `.env` in the root directory and provide all of its variables. The most important ones are:
- `DISCORD_TOKEN`
- `DATABASE_URL` (e.g., `postgres://username:password@localhost/autumn`)
- `DISCORD_GUILD_ID`
- `OLLAMA_HOST` (e.g., `http://127.0.0.1`)
- `OLLAMA_PORT` (e.g., `11434`)
- `OLLAMA_MODEL` (e.g., `llama3`)

Optional Redis cache variables:
- `REDIS_ENABLED` (`true`/`false`, default: `false`)
- `REDIS_URL` (e.g., `redis://127.0.0.1:6379`)
- `REDIS_KEY_PREFIX` (default: `autumn:prod`)

If Redis is enabled but unavailable or misconfigured, Autumn automatically falls back to database-only mode.

LLM mention requests are rate-limited per guild/channel/user (`2 requests / 10 seconds`). With Redis enabled, this limit is shared across multiple bot instances; without Redis it still works per process.

Optional LLM rate-limit overrides:
- `LLM_RATELIMIT_WINDOW_SECONDS` (default: `10`)
- `LLM_RATELIMIT_MAX_HITS` (default: `2`)

*(Optional)* The bot comes with a default system prompt for the LLM integration. If you want to use a custom prompt, simply create a `SYSTEM_PROMPT.md` file in the root directory and write your custom instructions there.

Next, install `sqlx-cli` if you haven't already. You can do so with:
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

Then navigate to the `autumn-database` directory and migrate the database:
```bash
cd autumn-database
sqlx migrate run
```
*This command will complain if the `DATABASE_URL` variable in `.env` is not correct.*

And finally, you can compile and run the bot with:
```bash
cargo run
```
To make the bot run faster (but compiling takes longer), use:
```bash
cargo run --release
```

## Containerized Setup (Docker / Podman)

A multi-stage `Dockerfile` and `docker-compose.yml` are included for running Autumn in containers.
The stack is compatible with **Docker Compose v2+** and **rootless Podman Compose** — it uses named volumes, runs the bot as an unprivileged user (uid 10001), and avoids any privileged flags.

### Services

| Service | Image | Notes |
|---|---|---|
| `postgres` | `postgres:16` | Required. Creates two DB roles on first init. |
| `redis` | `redis:7-alpine` | Enabled by default in the stack. |
| `migrator` | Built from repo | One-shot; runs `sqlx migrate run`. |
| `autumn-bot` | Built from repo | Connects as restricted `autumn_app` DB role. |
| `ollama` | `ollama/ollama` | **Opt-in** via `--profile llm`. |

The stack uses **two separate Postgres roles** for added safety:
- `autumn_migrator` — DDL-privileged; only used by the `migrator` service.
- `autumn_app` — DML-only (`SELECT`/`INSERT`/`UPDATE`/`DELETE`); used by the bot at runtime.

### First-time setup

**1. Create your `.env` file:**
```bash
cp .env.example .env
# Edit .env and fill in DISCORD_TOKEN, DISCORD_GUILD_ID, and the three Postgres passwords.
```

**2. Create `SYSTEM_PROMPT.md`** (leave empty to use the built-in default, or write your custom instructions):
```bash
touch SYSTEM_PROMPT.md
```

**3. Build images and start infrastructure:**
```bash
docker compose build
docker compose up -d postgres redis
```

**4. Run migrations** (required before first start and after every schema upgrade):
```bash
docker compose run --rm migrator
```

**5. Start the bot:**
```bash
docker compose up -d autumn-bot
```

### Updating the bot

Pull the latest code manually, then run the appropriate update script:

**Docker:**
```bash
git pull
./docker/update.sh

# With LLM profile:
COMPOSE_PROFILES=llm ./docker/update.sh
```

**Podman:**
```bash
git pull
./docker/update-podman.sh

# With LLM profile:
./docker/update-podman.sh llm
```

### Redis cache

Redis is **enabled by default** in the container stack (`REDIS_ENABLED=true`, `REDIS_URL=redis://redis:6379`). It provides:

- **LLM rate limiting shared across instances** — without Redis, rate limits are per-process only; with Redis they are enforced across all bot instances sharing the same key prefix.
- **In-memory caching** — reduces repeated database round-trips for hot paths.

The bot falls back to database-only mode automatically if Redis is unreachable, so it is safe to restart the Redis service independently.

To tune rate limits, add these to your `.env`:
```
LLM_RATELIMIT_WINDOW_SECONDS=10
LLM_RATELIMIT_MAX_HITS=2
REDIS_KEY_PREFIX=autumn:prod
```

To verify Redis is running:
```bash
docker compose exec redis redis-cli ping
# Expected: PONG
```

### Podman (rootless)

Use `podman-compose` (the native tool, not the Docker plugin):
```bash
# Install if not already present
sudo pacman -S podman-compose

podman-compose build
touch SYSTEM_PROMPT.md
podman-compose up -d postgres redis
podman-compose run --rm migrator
podman-compose up -d autumn-bot
```

With LLM (Ollama):
```bash
podman-compose --profile llm up -d postgres redis ollama
podman exec -it autumn_ollama_1 ollama pull llama3
podman-compose run --rm migrator
podman-compose --profile llm up -d autumn-bot
```

### Optional: LLM integration (Ollama)

Ollama is gated behind the `llm` compose profile to avoid pulling the large image by default.

```bash
# Start the full stack including Ollama
docker compose --profile llm up -d

# Or start Ollama separately to pull a model first
docker compose --profile llm up -d ollama
docker compose exec ollama ollama pull llama3

# Then start the rest
docker compose --profile llm up -d
```

Set `OLLAMA_MODEL` in your `.env` to the model name you pulled (e.g. `llama3`). The bot automatically disables LLM features if the Ollama service is unreachable.

### Optional: Custom system prompt

`SYSTEM_PROMPT.md` is always bind-mounted into the bot container at `/app/SYSTEM_PROMPT.md`. No compose changes needed — just edit the file in the repo root:

- **Empty file** (or `touch SYSTEM_PROMPT.md`) → uses the built-in default prompt.
- **File with content** → the bot uses your instructions instead.

Changes take effect on the next bot restart:
```bash
docker compose restart autumn-bot
```

---

*Note: This project originally started using the `twilight` ecosystem for Discord API interactions before being refactored to use `serenity` and `poise`. You can find the original archived repository here: [rusty-twilight](https://github.com/agneswd/rusty-twilight).*