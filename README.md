<!--
TideMark
========

File: README.md
Description: Top-level project overview and operator-facing command references.

Responsibility:
- Provide entry documentation for build, test, usage, and design links.

Architectural Position:
- Repository root documentation index.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# TideMark

<div align="center" style="margin: 20px 0;">
  <img src="./assets/logo.svg" width="120" height="120" alt="TideMark Logo" style="border-radius: 20px; box-shadow: 0 8px 32px rgba(0, 217, 255, 0.30);">
</div>

<h1 align="center">🚀 TideMark: Git Version Truth Layer</h1>
<p align="center"><strong>Deterministic, Git-native version coordinates for release gates and automation pipelines.</strong></p>

<p align="center">
  <a href='docs/TECHNICAL_DESIGN.md'><img src='https://img.shields.io/badge/Technical-Design-00d9ff?style=for-the-badge&labelColor=0f172a'></a>
  <a href='docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md'><img src='https://img.shields.io/badge/Category-Validation-22c55e?style=for-the-badge&labelColor=0f172a'></a>
  <a href='LICENSE'><img src='https://img.shields.io/badge/License-MIT-f97316?style=for-the-badge&labelColor=0f172a'></a>
</p>
<p align="center">
  <img src='https://img.shields.io/badge/Rust-2024-ce422b?style=for-the-badge&logo=rust&logoColor=white&labelColor=0f172a'>
  <img src='https://img.shields.io/badge/Tests-22_Passing-00d9ff?style=for-the-badge&labelColor=0f172a'>
  <img src='https://img.shields.io/badge/Determinism-Byte_Stable-22c55e?style=for-the-badge&labelColor=0f172a'>
</p>
<p align="center">
  <a href='docs/DELIVERY_REQUIREMENTS.md'><img src='https://img.shields.io/badge/Delivery-Requirements-8b5cf6?style=for-the-badge&labelColor=0f172a'></a>
  <a href='docs/DISTRIBUTION_AND_PLUGIN.md'><img src='https://img.shields.io/badge/Plugin_&_Distribution-Guide-ec4899?style=for-the-badge&labelColor=0f172a'></a>
  <a href='PROJECT_STRUCTURE.md'><img src='https://img.shields.io/badge/Project-Structure-eab308?style=for-the-badge&labelColor=0f172a'></a>
</p>

---

## Why TideMark

TideMark is the Git Version Truth Layer: a deterministic CLI that maps Git history and policy into a reproducible coordinate `x.y.z(.tag)`.

## Category Definition
- Cognitive anchor: Docker-style immutable trust identity, adapted to Git release coordinates.
- Category statement: TideMark defines and targets the `Git Version Truth Layer`.
- Falsifiable target: by December 31, 2026, TideMark should operate as a release gate in at least 20 public repositories.

## Why Not `git describe`?

`git describe` is useful for human-readable references, but it is not a deterministic release-gate contract by itself.

| Dimension | `git describe` (baseline) | TideMark |
|---|---|---|
| Output model | Description-oriented string | Protocol-like coordinate `x.y.z(.tag)` |
| Tie-breaking contract | Not designed for same-day index determinism policy | Explicit total-order anchor + same-day commit index ordering |
| Time policy | No built-in explicit timezone strategy for coordinate semantics | Explicit timezone policy (`UTC` or fixed offset) |
| Remote drift handling | Typically handled ad hoc in CI scripts | Built-in remote refresh semantics with typed fallback behavior |
| Failure surface | Generic command failure path | Typed error + stable exit code contract for automation |

Use TideMark when the version result is a release gate decision, not only a display label.

## Quick Start

```bash
cargo build --release
./target/release/tide mark --explain
```

## Core Commands

| Command | Description |
|---|---|
| `tide mark` | Resolve coordinate for `HEAD`. |
| `tide mark --explain` | Emit explainable key-value output. |
| `tide mark --local-only` | Disable remote refresh and use local tags only. |
| `tide file <path>` | Resolve coordinate for a file's last modifying commit. |
| `tide release list` | List release tags recognized by TideMark. |
| `tide config init` | Create `.tidemark.toml` with deterministic defaults. |
| `tide service plan` | Render deterministic systemd unit/timer text. |
| `tide service install` | Install and enable user-level timer (Linux). |
| `tide service uninstall` | Disable and remove user-level timer (Linux). |

## Git Plugin
- Binary `git-tide` is shipped together with `tide`.
- After installation, Git can invoke it as:
```bash
git tide mark
```

## Service Registration (Linux/systemd user)
```bash
tide service plan --interval-minutes 30
tide service install --interval-minutes 30
tide service uninstall
```

## Distribution
- Homebrew formula template: `packaging/homebrew/tide.rb.in`
- Debian build: `./scripts/release/build-deb.sh`
- APT publish helper: `./scripts/release/publish-apt.sh`
- Release tarball + checksums: `./scripts/release/build-dist.sh`

## Version Management and Auto Release
- `release-please` workflow (`.github/workflows/release-please.yml`) manages semantic version bumps, release PRs, tags, and changelog updates.
- After release publication (with tag `v*`), `release.yml` builds and uploads installable artifacts (`tar.gz`, checksum, `.deb`) to that release.
- Artifact version naming is normalized from the tag (`v0.1.0` -> `0.1.0`) before packaging.
- Recommended commit format for clean release notes: Conventional Commits (`feat:`, `fix:`, `chore:`).

## Build
```bash
cargo build
```

## Test
```bash
cargo test
```

## CI Gate Example

See [docs/CI_GATE_EXAMPLE.md](docs/CI_GATE_EXAMPLE.md) for a copy-paste GitHub Actions gate workflow.

## Design
See [docs/TECHNICAL_DESIGN.md](docs/TECHNICAL_DESIGN.md).
See [docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md](docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md).
See [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md).
See [docs/DELIVERY_REQUIREMENTS.md](docs/DELIVERY_REQUIREMENTS.md).
See [docs/DISTRIBUTION_AND_PLUGIN.md](docs/DISTRIBUTION_AND_PLUGIN.md).
