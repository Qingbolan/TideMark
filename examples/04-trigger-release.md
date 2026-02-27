<!--
TideMark
========

File: examples/04-trigger-release.md
Description: Practical examples for triggering TideMark automated release flows.

Responsibility:
- Show how to trigger release-please and release packaging workflows correctly.

Architectural Position:
- Release operations example for maintainers.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Example: Trigger Release

This repository uses:
- `release-please.yml` for version PR/tag/changelog automation.
- `release.yml` for artifact packaging and upload.

## Path A (Recommended): Trigger via `release-please`

1. Make a conventional commit on `main`:
```bash
git checkout main
git pull --ff-only
git commit --allow-empty -m "feat: add release trigger example"
git push origin main
```

2. Wait for `release-please` workflow to open or update a release PR.

3. Merge the release PR.  
   After merge, `release-please` creates a `v*` tag and GitHub Release.

4. `release.yml` runs on `release.published`, builds artifacts, and uploads:
- `tidemark-<version>-<target>.tar.gz`
- `tidemark-<version>-<target>.tar.gz.sha256`
- `.deb` (on Linux build job)

5. Optional package publishers run when credentials are configured:
- PyPI (`tidemark`)
- npm (`tidemark`)
- Homebrew tap update
- APT repository publish

## Path B: Manually Trigger Packaging for an Existing Tag

Use this when you need to rebuild artifacts for an existing release tag.

```bash
gh workflow run release.yml -f tag=v0.1.0
```

Check workflow runs:
```bash
gh run list --workflow release.yml
```

## Verification Commands

After release job completes, verify published assets in GitHub Release UI and optionally test local binary:

```bash
tide --version
git tide --version
```
