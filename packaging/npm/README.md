<!--
TideMark
========

File: packaging/npm/README.md
Description: Packaging notes for the npm TideMark launcher distribution.

Responsibility:
- Explain how the Node package resolves and runs release binaries.

Architectural Position:
- Ecosystem-facing documentation for JavaScript users.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# tidemark (npm)

`tidemark` installs `tide` and `git-tide` command wrappers.

On first install or first run, it downloads and verifies the matching release archive from GitHub Releases.

Environment variables:

- `TIDEMARK_GITHUB_REPOSITORY`: override release repository.
- `TIDEMARK_CACHE_DIR`: override local cache directory.
