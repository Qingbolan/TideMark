#!/usr/bin/env bash
# TideMark
# ========
#
# File: examples/03-remote-refresh.sh
# Description: Remote refresh and local-only divergence example.
#
# Responsibility:
# - Show how remote mode observes new remote tags while local-only remains stable.
#
# Architectural Position:
# - Runnable quickstart for remote strategy behavior.
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
REMOTE="$SANDBOX/remote.git"
UPSTREAM="$SANDBOX/upstream"
LOCAL="$SANDBOX/local"

git -C "$SANDBOX" init --bare remote.git >/dev/null

mkdir -p "$UPSTREAM"
git -C "$UPSTREAM" init -b main >/dev/null
git -C "$UPSTREAM" config user.name "TideMark Example"
git -C "$UPSTREAM" config user.email "example@tidemark.dev"

printf 'hello\n' > "$UPSTREAM/app.txt"
git -C "$UPSTREAM" add app.txt
GIT_AUTHOR_DATE="2024-01-01T00:00:00+00:00" \
GIT_COMMITTER_DATE="2024-01-01T00:00:00+00:00" \
  git -C "$UPSTREAM" commit -m "c1" >/dev/null

GIT_COMMITTER_DATE="2024-01-01T00:10:00+00:00" \
  git -C "$UPSTREAM" tag -a v1 -m "release 1" >/dev/null

git -C "$UPSTREAM" remote add origin "$REMOTE"
git -C "$UPSTREAM" push origin main --tags >/dev/null 2>&1

mkdir -p "$LOCAL"
git -C "$LOCAL" init -b main >/dev/null
git -C "$LOCAL" remote add origin "$REMOTE"
git -C "$LOCAL" fetch --tags origin main >/dev/null 2>&1
git -C "$LOCAL" checkout -B main FETCH_HEAD >/dev/null 2>&1

echo "Repository: $LOCAL"
echo "Expected initial mark: 1.0.0"
(
  cd "$ROOT_DIR"
  (cd "$LOCAL" && eval "$TIDE_CMD mark")
)

GIT_COMMITTER_DATE="2024-01-01T00:20:00+00:00" \
  git -C "$UPSTREAM" tag -a v2 -m "release 2" >/dev/null
git -C "$UPSTREAM" push origin v2 >/dev/null 2>&1

echo
echo "After pushing new remote tag v2:"
echo "Expected remote-mode mark: 2.0.0"
echo "Expected local-only mark: 1.0.0"
(
  cd "$ROOT_DIR"
  (cd "$LOCAL" && eval "$TIDE_CMD mark")
  (cd "$LOCAL" && eval "$TIDE_CMD mark --local-only")
)
