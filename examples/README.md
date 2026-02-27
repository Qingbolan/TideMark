<!--
TideMark
========

File: examples/README.md
Description: Runnable examples demonstrating practical TideMark usage patterns.

Responsibility:
- Provide copy-paste scenarios for local resolution, file resolution, and remote refresh behavior.

Architectural Position:
- Hands-on onboarding surface for command behavior validation.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# TideMark Examples

These examples build temporary repositories with deterministic timestamps and tags, then run TideMark commands.

## Prerequisites

1. From repository root, build TideMark:
```bash
cargo build --release --bin tide
```
2. Optional: install globally:
```bash
cargo install --path . --force
```

## Run Examples

1. Basic deterministic mark:
```bash
./examples/01-basic-mark.sh
```
2. File coordinate and metadata suffix:
```bash
./examples/02-file-and-tag.sh
```
3. Remote refresh versus local-only:
```bash
./examples/03-remote-refresh.sh
```
4. Release trigger flows (TideMark-derived release + manual packaging):
```bash
cat ./examples/04-trigger-release.md
```
5. Ecosystem release publishing (PyPI/npm/Homebrew/APT):
```bash
cat ./examples/05-trigger-ecosystem-release.md
```

## Notes

- Scripts prefer `tide` from `PATH`.  
- If `tide` is not installed, they fall back to `cargo run --bin tide -- ...` from repository root.
