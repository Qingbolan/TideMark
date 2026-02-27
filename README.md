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

<div align="center">

<div style="margin: 20px 0;">
  <img src="./assets/logo.svg" width="120" height="120" alt="TideMark Logo" style="border-radius: 20px; box-shadow: 0 8px 32px rgba(0, 217, 255, 0.30);">
</div>

# 🚀 TideMark: Git Version Truth Layer

<p><strong>Deterministic, Git-native version coordinates for release gates and automation pipelines.</strong></p>

<div align="center">
  <div style="width: 100%; height: 2px; margin: 20px 0; background: linear-gradient(90deg, transparent, #00d9ff, transparent);"></div>
</div>

<div align="center">
  <div style="background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%); border-radius: 15px; padding: 25px; text-align: center;">
    <p>
      <a href='docs/TECHNICAL_DESIGN.md'><img src='https://img.shields.io/badge/📘Technical-Design-00d9ff?style=for-the-badge&labelColor=0f172a'></a>
      <a href='docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md'><img src='https://img.shields.io/badge/🧭Category-Validation-22c55e?style=for-the-badge&labelColor=0f172a'></a>
      <a href='LICENSE'><img src='https://img.shields.io/badge/⚖️License-MIT-f97316?style=for-the-badge&labelColor=0f172a'></a>
    </p>
    <p>
      <img src='https://img.shields.io/badge/🦀Rust-2024-ce422b?style=for-the-badge&logo=rust&logoColor=white&labelColor=0f172a'>
      <img src='https://img.shields.io/badge/✅Tests-22_Passing-00d9ff?style=for-the-badge&labelColor=0f172a'>
      <img src='https://img.shields.io/badge/🧪Determinism-Byte_Stable-22c55e?style=for-the-badge&labelColor=0f172a'>
    </p>
    <p>
      <a href='docs/DELIVERY_REQUIREMENTS.md'><img src='https://img.shields.io/badge/📦Delivery-Requirements-8b5cf6?style=for-the-badge&labelColor=0f172a'></a>
      <a href='docs/DISTRIBUTION_AND_PLUGIN.md'><img src='https://img.shields.io/badge/🔌Plugin_&_Distribution-Guide-ec4899?style=for-the-badge&labelColor=0f172a'></a>
      <a href='PROJECT_STRUCTURE.md'><img src='https://img.shields.io/badge/🏗️Project-Structure-eab308?style=for-the-badge&labelColor=0f172a'></a>
    </p>
  </div>
</div>

</div>

---

## Why TideMark

TideMark is the Git Version Truth Layer: a deterministic CLI that maps Git history and policy into a reproducible coordinate `x.y.z(.tag)`.

## Category Definition
- Cognitive anchor: Docker-style immutable trust identity, adapted to Git release coordinates.
- Category statement: TideMark defines and targets the `Git Version Truth Layer`.
- Falsifiable target: by December 31, 2026, TideMark should operate as a release gate in at least 20 public repositories.

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

## Build
```bash
cargo build
```

## Test
```bash
cargo test
```

## Design
See [docs/TECHNICAL_DESIGN.md](docs/TECHNICAL_DESIGN.md).
See [docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md](docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md).
See [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md).
See [docs/DELIVERY_REQUIREMENTS.md](docs/DELIVERY_REQUIREMENTS.md).
See [docs/DISTRIBUTION_AND_PLUGIN.md](docs/DISTRIBUTION_AND_PLUGIN.md).
