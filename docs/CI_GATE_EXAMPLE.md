<!--
TideMark
========

File: docs/CI_GATE_EXAMPLE.md
Description: Copy-paste GitHub Actions examples for using TideMark as a deterministic release gate.

Responsibility:
- Provide practical CI wiring for coordinate checks and typed failure handling.

Architectural Position:
- CI integration guide for automation adoption.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# CI Gate Example

## Example 1: Require Same-Day Release (`y == 0`)

This workflow fails when `tide mark --explain` reports `day_delta != 0`.

```yaml
name: tidemark-gate

on:
  pull_request:
  push:
    branches: [main]

jobs:
  gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build TideMark
        run: cargo build --release --bin tide
      - name: Resolve mark (explain)
        id: mark
        run: |
          OUT="$(./target/release/tide mark --explain)"
          echo "$OUT"
          DAY_DELTA="$(echo "$OUT" | awk -F= '$1=="day_delta"{print $2}')"
          echo "day_delta=$DAY_DELTA" >> "$GITHUB_OUTPUT"
      - name: Gate on policy
        run: |
          if [ "${{ steps.mark.outputs.day_delta }}" != "0" ]; then
            echo "Policy failed: day_delta must be 0"
            exit 1
          fi
```

## Example 2: Branch-Suffixed Coordinate in CI

This workflow enforces a metadata suffix in CI output.

```yaml
name: tidemark-branch-suffix

on:
  pull_request:

jobs:
  gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release --bin tide
      - name: Resolve branch-tagged coordinate
        run: |
          BRANCH="${GITHUB_HEAD_REF:-${GITHUB_REF_NAME}}"
          ./target/release/tide mark --tag "$BRANCH"
```

## Failure Handling Notes

- TideMark uses typed exit codes, so CI can distinguish configuration errors from data-state errors.
- For strict local-only mode in CI, add `--local-only`.
- For remote mode, TideMark refreshes remote release tags and applies configured fallback behavior.
