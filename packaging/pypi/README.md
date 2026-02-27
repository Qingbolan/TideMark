<!--
TideMark
========

File: packaging/pypi/README.md
Description: Packaging notes for the PyPI TideMark launcher distribution.

Responsibility:
- Explain how the Python package resolves and runs release binaries.

Architectural Position:
- Ecosystem-facing documentation for Python users.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# tidemark (PyPI)

`tidemark` installs lightweight launchers for `tide` and `git-tide`.

On first run, the launcher downloads the matching release archive from GitHub Releases and caches the binaries locally.

Environment variables:

- `TIDEMARK_GITHUB_REPOSITORY`: override release repository, default is the upstream repository.
- `TIDEMARK_CACHE_DIR`: override local cache directory.
