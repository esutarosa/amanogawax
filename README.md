## Project layout
```
.
├─ Cargo.toml        # workspace manifest
├─ packages/
│  ├─ web/           # wasm target + static assets
│  ├─ desktop/       # native desktop binary
│  ├─ mobile/        # mobile entrypoint (simulator/devices)
│  └─ ui/            # shared components, styling, assets
└─ target/           # cargo build artifacts
```
Each platform crate exposes `src/main.rs`, platform-specific `assets/`, and view modules under `src/views/`. The `ui` crate must stay platform agnostic; only import it from platform crates.

## Development commands
Run these from the repo root:

| Task | Command | Notes |
| --- | --- | --- |
| Web dev server | `cd packages/web && dx serve --platform web --package web` | Hot reloads + opens browser. |
| Desktop dev | `cd packages/desktop && dx serve --platform desktop --package desktop` | Runs native window with live reload. |
| Mobile dev (sim/device) | `cd packages/mobile && dx serve --platform mobile --package mobile` | Requires matching SDK/emulator. |
| Workspace check | `cargo fmt --all && cargo clippy --all-targets --all-features` | Keep formatting and lints clean. |

## Building
Produce optimized artifacts per target:

```bash
# web (wasm bundle)
cd packages/web
dx build --platform web --package web --release

# desktop native binary
cd packages/desktop
dx build --platform desktop --package desktop --release
```
Mobile builds follow the same pattern but require the appropriate toolchain (Xcode, Android SDK) configured beforehand.

## Testing
Run Rust tests per crate or for the whole workspace:
```bash
cargo test --workspace
cargo test -p ui            # target a specific crate
```
Place shared component tests inside `packages/ui/src` modules with `#[cfg(test)]` blocks; platform-specific integration tests belong in each crate’s `tests/` directory.
