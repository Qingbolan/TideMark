#!/usr/bin/env bash
# TideMark
# ========
#
# File: examples/02-file-and-tag.sh
# Description: File coordinate and metadata suffix example.
#
# Responsibility:
# - Demonstrate `tide file` and tagged mark output formatting.
#
# Architectural Position:
# - Runnable quickstart for file-history resolution and suffix output.
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
REPO="$SANDBOX/file-and-tag"
mkdir -p "$REPO"

git -C "$REPO" init -b main >/dev/null
git -C "$REPO" config user.name "TideMark Example"
git -C "$REPO" config user.email "example@tidemark.dev"

printf 'a1\n' > "$REPO/a.txt"
git -C "$REPO" add a.txt
GIT_AUTHOR_DATE="2024-01-01T00:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-01T00:00:00+00:00" \
  git -C "$REPO" commit -m "c1" >/dev/null

GIT_COMMITTER_DATE="2024-01-01T00:05:00+00:00" \
  git -C "$REPO" tag -a v3 -m "release 3" >/dev/null

printf 'a2\n' > "$REPO/a.txt"
git -C "$REPO" add a.txt
GIT_AUTHOR_DATE="2024-01-01T01:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-01T01:00:00+00:00" \
  git -C "$REPO" commit -m "c2" >/dev/null

printf 'b1\n' > "$REPO/b.txt"
git -C "$REPO" add b.txt
GIT_AUTHOR_DATE="2024-01-02T01:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-02T01:00:00+00:00" \
  git -C "$REPO" commit -m "c3" >/dev/null

echo "Repository: $REPO"
echo "Expected file coordinate for a.txt: 3.0.1"
echo "Expected tagged head coordinate: 3.1.1.dev"
echo
(
  cd "$ROOT_DIR"
  (cd "$REPO" && eval "$TIDE_CMD file a.txt --local-only")
  (cd "$REPO" && eval "$TIDE_CMD mark --local-only --tag dev")
)
