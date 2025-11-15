# Release Template (Pride Versioning)

We use a three-part pride versioning scheme: `PROUD.DEFAULT.SHAME`.

- `PROUD` — bump when the release is a milestone you are proud of (new platforms, flagship features, major UX leaps).
- `DEFAULT` — bump for normal/okay releases that move the roadmap forward without drama.
- `SHAME` — bump when hotfixing regressions or embarrassing mistakes; expect short-lived tags and quick follow-ups.

## Example Tag

```
2.7.123
│ │ └─ shame hotfix count
│ └── default cadence (e.g., sprint)
└──── proud milestone level
```

## Release Notes Template

```
## AmanogawaX v{PROUD}.{DEFAULT}.{SHAME} — {codename or emoji}

### Highlights
- Proud reason (why did we bump PROUD? what makes this special?)

### Added
- New features, endpoints, platforms, widgets

### Changed
- UX tweaks, refactors, dependency bumps

### Fixed
- Bug numbers / links to issues

### Known Issues
- Outstanding problems; link to tracking issues

### Checks
- [ ] `cargo fmt --all`
- [ ] `cargo clippy --all-targets --all-features`
- [ ] `cargo test --workspace`
- [ ] `dx serve` smoke verified on web/desktop
- [ ] Artifacts uploaded (web bundle, desktop build, API image)
```

Add screenshots/GIFs for UI releases and attach telemetry/benchmark diffs when relevant.
