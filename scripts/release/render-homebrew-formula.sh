#!/usr/bin/env bash
# TideMark
# ========
#
# File: scripts/release/render-homebrew-formula.sh
# Description: Render Homebrew formula file from template placeholders.
#
# Responsibility:
# - Materialize versioned formula with URL and checksum for distribution.
#
# Architectural Position:
# - Template renderer in Homebrew packaging workflow.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

if [[ $# -lt 5 ]]; then
  echo "usage: $0 <version-without-v> <x64-url> <x64-sha256> <arm64-url> <arm64-sha256> [repository]" >&2
  exit 2
fi

VERSION="$1"
X64_URL="$2"
X64_SHA256="$3"
ARM64_URL="$4"
ARM64_SHA256="$5"
REPOSITORY="${6:-Qingbolan/TideMark}"
REPO_OWNER="${REPOSITORY%%/*}"
REPO_NAME="${REPOSITORY##*/}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMPLATE="$ROOT_DIR/packaging/homebrew/tide.rb.in"
OUTPUT="$ROOT_DIR/packaging/homebrew/tide.rb"

sed \
  -e "s|<VER>|$VERSION|g" \
  -e "s|<X64_URL>|$X64_URL|g" \
  -e "s|<X64_SHA256>|$X64_SHA256|g" \
  -e "s|<ARM64_URL>|$ARM64_URL|g" \
  -e "s|<ARM64_SHA256>|$ARM64_SHA256|g" \
  -e "s|<ORG>|$REPO_OWNER|g" \
  -e "s|<REPO>|$REPO_NAME|g" \
  "$TEMPLATE" > "$OUTPUT"

echo "formula_path=$OUTPUT"
