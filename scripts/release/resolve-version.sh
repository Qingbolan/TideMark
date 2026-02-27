#!/usr/bin/env bash
# TideMark
# ========
#
# File: scripts/release/resolve-version.sh
# Description: Resolve release version from current repository state using TideMark coordinates.
#
# Responsibility:
# - Compute deterministic `x.y.z(.tag)` coordinate and emit a release-safe `x.y.z` version.
#
# Architectural Position:
# - Shared release helper for tag/release automation workflows.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

ARGS=(mark --local-only)
if [[ "${TIDEMARK_ALLOW_REMOTE:-false}" == "true" ]]; then
  ARGS=(mark)
fi

set +e
RESOLVE_OUTPUT="$(cargo run --quiet --bin tide -- "${ARGS[@]}" 2>&1)"
RESOLVE_CODE=$?
set -e

if [[ $RESOLVE_CODE -ne 0 ]]; then
  if [[ -n "${TIDEMARK_BOOTSTRAP_VERSION:-}" ]] && [[ "$RESOLVE_OUTPUT" == *"no release anchor found"* ]]; then
    if [[ ! "${TIDEMARK_BOOTSTRAP_VERSION}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
      echo "error: invalid bootstrap version: ${TIDEMARK_BOOTSTRAP_VERSION}" >&2
      exit 4
    fi
    COORDINATE="${TIDEMARK_BOOTSTRAP_VERSION}"
  else
    echo "$RESOLVE_OUTPUT" >&2
    exit "$RESOLVE_CODE"
  fi
else
  COORDINATE="$RESOLVE_OUTPUT"
fi

if [[ ! "$COORDINATE" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.][A-Za-z0-9._-]+)?$ ]]; then
  echo "error: invalid coordinate returned by TideMark: $COORDINATE" >&2
  exit 3
fi

BASE_VERSION="$(printf '%s' "$COORDINATE" | cut -d'.' -f1-3)"

echo "coordinate=$COORDINATE"
echo "version=$BASE_VERSION"
