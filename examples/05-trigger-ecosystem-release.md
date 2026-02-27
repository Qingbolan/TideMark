<!--
TideMark
========

File: examples/05-trigger-ecosystem-release.md
Description: Practical example for triggering multi-ecosystem package publication from a release event.

Responsibility:
- Show copy-paste steps for publishing GitHub assets, PyPI, npm, Homebrew, and APT in one flow.

Architectural Position:
- Release operations example focused on package manager fan-out.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Example: Trigger Ecosystem Release

This example assumes `.github/workflows/release.yml` is enabled and credentials are configured.

## 1. Configure required credentials

Repository settings:

- Secrets:
  - `PYPI_API_TOKEN` (optional, for PyPI publishing)
  - `NPM_TOKEN` (optional, for npm publishing)
  - `HOMEBREW_TAP_TOKEN` (optional, for Homebrew tap updates)
  - `APT_GPG_PRIVATE_KEY` and `APT_GPG_PASSPHRASE` (optional, for signed APT metadata)
- Variables:
  - `HOMEBREW_TAP_REPOSITORY` (optional, for Homebrew tap target)
  - `APT_ENABLE_PUBLISH=true` (optional, enable APT publish job)
  - `APT_GH_PAGES_BRANCH` (optional, default `gh-pages`)
  - `APT_DISTRIBUTION` (optional, default `stable`)
  - `APT_COMPONENT` (optional, default `main`)
  - `APT_GPG_KEY_ID` (optional, when signing APT metadata)

## 2. Trigger TideMark-derived release (recommended)

```bash
git checkout main
git pull --ff-only
gh workflow run release-from-tidemark.yml -f ref=main
```

Then GitHub release publication triggers `release.yml`.

If this is the first release and no anchor tag exists:

```bash
gh workflow run release-from-tidemark.yml -f ref=main -f bootstrap_version=0.1.0
```

## 3. Manual trigger for an existing tag

```bash
gh workflow run release.yml -f tag=v0.1.0
```

Track run status:

```bash
gh run list --workflow release.yml
gh run view <run-id> --log
```

## 4. Verify publication outputs

- GitHub release assets include archives, checksums, and `.deb`.
- PyPI:
  - `pip install tidemark`
- npm:
  - `npm install -g tidemark`
- Homebrew:
  - `brew install <tap>/tide`
- APT:
  - `https://<owner>.github.io/<repo>/apt`
