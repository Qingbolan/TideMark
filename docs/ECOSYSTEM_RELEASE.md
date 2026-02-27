<!--
TideMark
========

File: docs/ECOSYSTEM_RELEASE.md
Description: End-to-end release publication guide for GitHub assets, PyPI, npm, Homebrew, and APT.

Responsibility:
- Define workflow triggers, required credentials, and consumer installation endpoints.

Architectural Position:
- Release operations contract for multi-ecosystem distribution.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Ecosystem Release Guide

## Purpose

TideMark release automation is designed for agent-first delivery:

- build deterministic native artifacts,
- publish artifacts to GitHub Release,
- publish launcher packages to PyPI and npm,
- update Homebrew tap formula,
- publish a static APT repository tree.

All publication stages are executed by `.github/workflows/release.yml`.

## Trigger

Recommended path:

1. Push conventional commits to `main`.
2. `release-please` opens a release PR.
3. Merge the release PR.
4. GitHub publishes tag `vX.Y.Z`, then `release.yml` runs automatically.

Manual fallback:

```bash
gh workflow run release.yml -f tag=v0.1.0
```

## Required Secrets and Variables

### Always required

- `GITHUB_TOKEN` (provided by Actions automatically).

### Optional ecosystem publishers

- PyPI:
  - Secret: `PYPI_API_TOKEN`
- npm:
  - Secret: `NPM_TOKEN`
- Homebrew tap:
  - Secret: `HOMEBREW_TAP_TOKEN`
  - Variable: `HOMEBREW_TAP_REPOSITORY` (for example `your-org/homebrew-tap`)
- APT (disabled by default):
  - Variable: `APT_ENABLE_PUBLISH` set to `true`
  - Optional variable: `APT_GH_PAGES_BRANCH` (default `gh-pages`)
  - Optional variable: `APT_DISTRIBUTION` (default `stable`)
  - Optional variable: `APT_COMPONENT` (default `main`)
  - Optional signing:
    - Variable: `APT_GPG_KEY_ID`
    - Secret: `APT_GPG_PRIVATE_KEY`
    - Secret: `APT_GPG_PASSPHRASE`

If optional credentials are missing, the corresponding ecosystem stage is skipped.

## Published Artifacts

Each release publishes:

- `tidemark-<version>-x86_64-unknown-linux-gnu.tar.gz`
- `tidemark-<version>-aarch64-unknown-linux-gnu.tar.gz`
- `tidemark-<version>-x86_64-apple-darwin.tar.gz`
- `tidemark-<version>-aarch64-apple-darwin.tar.gz`
- checksum files for each archive
- `.deb` package artifact

## Consumer Installation Endpoints

- GitHub Release assets: direct binary download.
- PyPI package: `pip install tidemark`.
- npm package: `npm install -g tidemark`.
- Homebrew: `brew install <tap>/tide`.
- APT: repository served from `https://<owner>.github.io/<repo>/apt`.
