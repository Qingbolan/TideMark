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

if [[ $# -lt 3 ]]; then
  echo "usage: $0 <version-without-v> <archive-url> <sha256>" >&2
  exit 2
fi

VERSION="$1"
ARCHIVE_URL="$2"
SHA256="$3"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMPLATE="$ROOT_DIR/packaging/homebrew/tide.rb.in"
OUTPUT="$ROOT_DIR/packaging/homebrew/tide.rb"

sed \
  -e "s|<VER>|$VERSION|g" \
  -e "s|<URL>|$ARCHIVE_URL|g" \
  -e "s|<SHA256>|$SHA256|g" \
  "$TEMPLATE" > "$OUTPUT"

echo "formula_path=$OUTPUT"
