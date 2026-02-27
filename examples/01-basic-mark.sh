#!/usr/bin/env bash
# TideMark
# ========
#
# File: examples/01-basic-mark.sh
# Description: Minimal deterministic mark example for local-only resolution.
#
# Responsibility:
# - Build a fixture repository and demonstrate `tide mark` and `tide mark --explain`.
#
# Architectural Position:
# - Runnable quickstart for commit-coordinate semantics.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

resolve_tide_cmd() {
  if command -v tide >/dev/null 2>&1; then
    printf 'tide'
  else
    printf 'cargo run --quiet --bin tide --'
  fi
}

TIDE_CMD="$(resolve_tide_cmd)"

SANDBOX="$(mktemp -d)"
REPO="$SANDBOX/basic-mark"
mkdir -p "$REPO"

git -C "$REPO" init -b main >/dev/null
git -C "$REPO" config user.name "TideMark Example"
git -C "$REPO" config user.email "example@tidemark.dev"

printf 'a\n' > "$REPO/app.txt"
git -C "$REPO" add app.txt
GIT_AUTHOR_DATE="2024-01-01T00:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-01T00:00:00+00:00" \
  git -C "$REPO" commit -m "c1" >/dev/null

GIT_COMMITTER_DATE="2024-01-01T00:10:00+00:00" \
  git -C "$REPO" tag -a v1 -m "release 1" >/dev/null

printf 'b\n' > "$REPO/app.txt"
git -C "$REPO" add app.txt
GIT_AUTHOR_DATE="2024-01-01T01:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-01T01:00:00+00:00" \
  git -C "$REPO" commit -m "c2" >/dev/null

printf 'c\n' > "$REPO/app.txt"
git -C "$REPO" add app.txt
GIT_AUTHOR_DATE="2024-01-02T01:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-02T01:00:00+00:00" \
  git -C "$REPO" commit -m "c3" >/dev/null

echo "Repository: $REPO"
echo "Expected coordinate: 1.1.1"
echo
(
  cd "$ROOT_DIR"
  (cd "$REPO" && eval "$TIDE_CMD mark --local-only")
  echo
  (cd "$REPO" && eval "$TIDE_CMD mark --local-only --explain")
)
