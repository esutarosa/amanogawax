# Repository Guidelines
A concise guide for developers and integrator agents working on the Amanogawax project.

## Project Structure & Module Organization
This Cargo workspace contains platform crates in `packages/web`, `packages/desktop`, and `packages/mobile`, plus shared UI in `packages/ui`. Each platform crate keeps its own `assets/` and `src/` trees; entry points live in `src/main.rs`, while view modules live under `src/views/`. Shared components (for example `ui/src/hero.rs` or `ui/src/navbar.rs`) should remain platform-agnostic and reference styling tucked under `ui/assets/`. Keep platform-only code out of the shared crate to prevent dependency leaks.

## Build, Test, and Development Commands
- `cargo check -p <crate>` validates a specific package quickly; use `cargo check -p ui` before re-exporting shared widgets.
- `dx serve` (run from a platform directory) launches the hot-reload dev server; e.g. `cd packages/web && dx serve`.
- `cargo build --release -p desktop` produces optimized artifacts for the desktop app; swap `desktop` for other crates as needed.
- `cargo fmt --all` and `cargo clippy --all-targets --all-features` enforce formatting and linting prior to review.

## Coding Style & Naming Conventions
Adopt Rust 2021 defaults: four-space indentation, snake_case for modules/functions, UpperCamelCase for types, and SCREAMING_SNAKE_CASE for constants. Keep modules short (≤300 lines) and prefer splitting view logic into files such as `views/blog.rs`. Re-export view modules through `views/mod.rs` to keep route declarations concise.

## Testing Guidelines
Use `cargo test -p <crate>` for unit tests; prefer colocating tests via Rust’s `#[cfg(test)]` modules beneath the function under test. Integration tests belong in `<crate>/tests/`. Name tests descriptively (`renders_navbar_with_links`) and ensure new shared UI components ship with coverage in the `ui` crate. Aim to keep `cargo test --workspace` green before submitting PRs.

## Commit & Pull Request Guidelines
Commits follow conventional prefixes (`feat:`, `fix:`, `docs:`) because the history is currently empty—set that precedent now. Keep commits scoped to one logical change and include crate names when relevant (`feat(web): add blog route`). Pull requests should explain the problem, summarize the solution, link any tracking issues, and attach screenshots or GIFs for UI-facing updates on each platform impacted. Include test or `dx serve` output snippets that demonstrate the change working.


## Overview

Amanogawa is a multi‑platform anime client (Web/Desktop/Mobile) with a Rust backend that ingests data from a Telegram bot. UI is built with **Dioxus** (+ **dioxus‑motion** for animations). Server uses **Axum**. Architecture is **FSD for UI** + **Hexagonal/Clean** for backend.

> Note: Tailwind has been removed. Use plain CSS or your preferred styling approach (e.g., CSS files, utility classes you define, or a future design system). Avoid bundling secrets in the repo.

## Tech Stack

* **Rust** (Cargo workspace)
* **Dioxus 0.6** (web/desktop; mobile can be enabled later)
* **dioxus‑motion** (declarative animations)
* **Axum**, **tokio**, **serde**, **tower‑http**
* (optional) **SQLx + Postgres**, **tracing**

## Monorepo & Layers

Suggested layout (matches the current workspace created by `dx new … Workspace`):

```
packages/
  web/         # Dioxus Web (entrypoint)
  desktop/     # Dioxus Desktop (entrypoint)
  mobile/      # (optional) Dioxus iOS/Android later
  ui/          # UI by FSD: pages/widgets/features/shared
  api/         # Axum server (Telegram webhook + REST/GraphQL)
crates/        # Domain & infrastructure crates (Hex)
  domain/      # entities, value objects, invariants
  usecases/    # application services (interactors)
  ports/       # trait-based ports: Repos, TelegramGateway, MediaCdn …
  infra-db/    # DB adapter (SQLx/SeaORM), implements ports
  infra-tg/    # Telegram adapter (webhook/client), implements ports
  contracts/   # shared DTOs / serde types for API <-> UI
  http-client/ # typed client for UI to call API
```

**Dependency rule (Hex):**
`domain <- usecases <- (api, infra-*)`. UI does **not** depend on `domain/usecases`; it consumes HTTP via `contracts/http-client`.

## UI by FSD (packages/ui)

```
src/
  shared/      # design tokens, base components, hooks, utils
  entities/    # view-models (UI only), formatting, i18n helpers
  views/
    releases/
    title/
    episode/
  widgets/     # Navbar, Player, Grid, etc.
  features/    # add_to_favorites, change_quality, filters, etc.
```

Keep most modules `pub(crate)` and re-export only the necessary public API.

## Data Flow

Telegram Bot → `packages/api` (`POST /webhook/telegram/:secret`) → DB → `/v1/*` endpoints → UI (web/desktop/mobile). For the player, use HTML5 `<video>` with Shaka/hls.js and a Rust⇄JS bridge for controls (quality/speed/subtitles/PiP/FS).

## Quick Start (Dev)

### Web

```bash
cd packages/web
dx serve
```

### Desktop (macOS)

```bash
cd packages/desktop
dx serve --platform desktop
```

### API

```bash
cargo run -p amanogawa-api
# health: http://localhost:8787/healthz
```

## Configuration & Secrets (".env access is blocked")

* Do **not** rely on reading local `.env` files at runtime.
* Provide configuration via **OS environment variables** or the process manager (systemd, Docker, CI). Example:

  * macOS/Linux (temporary for one command):

    ```bash
    TELEGRAM_WEBHOOK_SECRET=your-secret cargo run -p amanogawax-api
    ```
  * macOS/Linux (export for the shell):

    ```bash
    export TELEGRAM_WEBHOOK_SECRET=your-secret
    cargo run -p amanogawax-api
    ```
  * Windows PowerShell:

    ```powershell
    $env:TELEGRAM_WEBHOOK_SECRET = "your-secret"
    cargo run -p amanogawax-api
    ```
* For CI/CD, store secrets in the platform’s secret manager (GitHub Actions, GitLab, etc.).
* Avoid committing any credentials. If needed, keep a non-secret `config.example.toml` for reference.

**Telegram webhook setup (prod or via tunnel):**

```
https://api.telegram.org/bot<YOUR_BOT_TOKEN>/setWebhook?url=https://<domain>/webhook/telegram/<TELEGRAM_WEBHOOK_SECRET>
```

## Quality & Style

* `rustfmt`, `clippy` clean
* Conventional Commits (`feat:`, `fix:`, `chore:` …)
* Logging via `tracing` (`info`, `warn`, `error`)
* Unit tests in `domain/usecases`, integration tests in `api`

## Build Targets

```bash
# Web (prod)
cd packages/web && dx build

# Desktop (prod)
cd packages/desktop && dx build --platform desktop

# Mobile will require platform SDKs (Xcode/Android Studio) when enabled.
```

## Security

* Secrets only via environment or CI Secret Manager
* Narrow CORS in API to known frontends
* Signed URLs for media/CDN (anti-hotlink)
