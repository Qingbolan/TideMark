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
- A Formula defines binary URLs, SHA256, and install logic.
- TideMark renders and pushes a formula to a dedicated tap repository during release automation.

Official reference:
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)

### APT
- APT distribution depends on Debian metadata (`Packages`, `Release`, optional `InRelease`) plus `.deb` artifacts.
- TideMark generates a static repository tree and publishes it through GitHub Pages.

Official reference:
- [Debian Wiki: Repository Format](https://wiki.debian.org/DebianRepository/Format)

### PyPI and npm
- PyPI package `tidemark` installs Python launchers that resolve release binaries.
- npm package `tidemark` installs Node launchers with the same release-resolution behavior.

## 4. Implemented Release Assets
- Homebrew template: `packaging/homebrew/tide.rb.in`
- Formula render script: `scripts/release/render-homebrew-formula.sh`
- Debian build path: `scripts/release/build-deb.sh` + `Cargo.toml` `package.metadata.deb`
- APT repository builder: `scripts/release/build-apt-repo.sh`
- PyPI package source: `packaging/pypi/`
- npm package source: `packaging/npm/`
- Multi-platform release workflow: `.github/workflows/release.yml`
- TideMark-derived release trigger workflow: `.github/workflows/release-from-tidemark.yml`
- Optional semantic-release helper workflow: `.github/workflows/release-please.yml`

## 5. Upgrade Path
- Homebrew: `brew upgrade tide`
- APT: `apt update && apt upgrade tidemark`
- PyPI: `pip install --upgrade tidemark`
- npm: `npm install -g tidemark@latest`

Note: There is no in-place binary self-update command. Upgrades are managed through package managers or GitHub Releases.

## 6. Automated Release Flow

1. Trigger `release-from-tidemark.yml` from `main`.
2. Workflow computes TideMark version and creates a `v*` tag/GitHub Release.
3. `release.yml` is triggered by release publication, normalizes `v*` to plain semantic version, then:
   - builds release archives and `.deb`,
   - uploads GitHub assets,
   - publishes PyPI and npm packages when credentials exist,
   - updates Homebrew tap and publishes APT repository when enabled.

## 7. Formula Rendering Example
```bash
./scripts/release/render-homebrew-formula.sh \
  0.1.0 \
  https://github.com/<ORG>/<REPO>/releases/download/v0.1.0/tidemark-0.1.0-x86_64-apple-darwin.tar.gz \
  <x64-sha256> \
  https://github.com/<ORG>/<REPO>/releases/download/v0.1.0/tidemark-0.1.0-aarch64-apple-darwin.tar.gz \
  <arm64-sha256> \
  <ORG>/<REPO>
```
