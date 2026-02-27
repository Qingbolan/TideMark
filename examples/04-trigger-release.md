<!--
TideMark
========

File: examples/04-trigger-release.md
Description: Practical examples for triggering TideMark automated release flows.

Responsibility:
- Show how to trigger TideMark-derived release and release packaging workflows correctly.

Architectural Position:
- Release operations example for maintainers.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Example: Trigger Release

This repository uses:
- `release-from-tidemark.yml` for TideMark-derived tag/release creation.
- `release.yml` for artifact packaging and upload.

## Path A (Recommended): Trigger via TideMark-derived release

1. Ensure your target commit is on `main`:
```bash
git checkout main
git pull --ff-only
```

2. Trigger release creation:
```bash
gh workflow run release-from-tidemark.yml -f ref=main
```

If this is the first release and no release anchor exists yet:
```bash
gh workflow run release-from-tidemark.yml -f ref=main -f bootstrap_version=0.1.0
```

3. `release.yml` runs on `release.published`, builds artifacts, and uploads:
- `tidemark-<version>-<target>.tar.gz`
- `tidemark-<version>-<target>.tar.gz.sha256`
- `.deb` (on Linux build job)

4. Optional package publishers run when credentials are configured:
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
