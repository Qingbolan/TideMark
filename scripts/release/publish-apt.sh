#!/usr/bin/env bash
# TideMark
# ========
#
# File: scripts/release/publish-apt.sh
# Description: Publish TideMark Debian artifacts to an APT repository via aptly.
#
# Responsibility:
# - Add package, snapshot repository state, and publish or switch distribution.
#
# Architectural Position:
# - External repository publication helper for APT distribution.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

# Requires aptly: https://www.aptly.info/
# Example:
#   ./scripts/release/publish-apt.sh dist/tidemark_0.1.0-1_amd64.deb stable

if [[ $# -lt 2 ]]; then
  echo "usage: $0 <deb-file> <distribution>"
  exit 2
fi

DEB_FILE="$1"
DIST="$2"
REPO_NAME="tidemark"

if ! command -v aptly >/dev/null 2>&1; then
  echo "error: aptly is required" >&2
  exit 3
fi

set +e
aptly repo show "$REPO_NAME" >/dev/null 2>&1
HAS_REPO=$?
set -e

if [[ $HAS_REPO -ne 0 ]]; then
  aptly repo create -distribution="$DIST" -component=main "$REPO_NAME"
fi

aptly repo add "$REPO_NAME" "$DEB_FILE"
SNAPSHOT="${REPO_NAME}-${DIST}-$(date +%Y%m%d%H%M%S)"
aptly snapshot create "$SNAPSHOT" from repo "$REPO_NAME"

set +e
aptly publish show "$DIST" >/dev/null 2>&1
HAS_PUBLISH=$?
set -e

if [[ $HAS_PUBLISH -eq 0 ]]; then
  aptly publish switch "$DIST" "$SNAPSHOT"
else
  aptly publish snapshot "$SNAPSHOT" "$DIST"
fi

echo "published_snapshot=$SNAPSHOT"
