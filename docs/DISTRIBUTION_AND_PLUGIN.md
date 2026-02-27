<!--
TideMark
========

File: docs/DISTRIBUTION_AND_PLUGIN.md
Description: Distribution and Git plugin notes for TideMark release and installation workflows.

Responsibility:
- Document plugin dispatch behavior and package-distribution strategy.

Architectural Position:
- Delivery operations reference for packaging and release engineering.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Git Plugin and Distribution Guide

## 1. Git Plugin Conclusion
TideMark can be exposed as a Git external subcommand plugin.

Rationale: According to the Git `git-help(1)` behavior around external commands, Git discovers executables named `git-*` from `PATH`. Therefore, shipping `git-tide` enables `git tide ...` invocation.

Official reference:
- [git-help(1)](https://git-scm.com/docs/git-help)

## 2. Plugin Implementation in This Project
- Cargo builds two binaries: `tide` and `git-tide`.
- Both binaries share the same application entry and business logic.
- Regression tests verify parity between `git-tide mark` and `tide mark`.

## 3. Distribution Research Summary
### Homebrew
- A Formula defines source/binary URL, SHA256, and install logic.
- Formula hosting is typically done in a dedicated tap repository, then installed with `brew install <tap>/tide`.

Official reference:
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)

### APT
- APT distribution depends on Debian repository metadata (`Packages` / `Release`) plus `.deb` artifacts.
- Repository lifecycle can be managed with `aptly` (repo/snapshot/publish).

Official reference:
- [Debian Wiki: Setup With Aptly](https://wiki.debian.org/DebianRepository/SetupWithAptly)

## 4. Implemented Release Assets
- Homebrew template: `packaging/homebrew/tide.rb.in`
- Formula render script: `scripts/release/render-homebrew-formula.sh`
- Debian build path: `scripts/release/build-deb.sh` + `Cargo.toml` `package.metadata.deb`
- APT publish helper: `scripts/release/publish-apt.sh`
- Multi-platform release workflow: `.github/workflows/release.yml`

## 5. Upgrade Path
- Homebrew: `brew upgrade tide`
- APT: `apt update && apt upgrade tidemark`

Note: There is no in-place binary self-update command. Upgrades are managed through package managers or GitHub Releases.

## 6. Formula Rendering Example
```bash
./scripts/release/render-homebrew-formula.sh \
  0.1.0 \
  https://github.com/<ORG>/<REPO>/releases/download/v0.1.0/tidemark-v0.1.0-aarch64-apple-darwin.tar.gz \
  <sha256>
```
