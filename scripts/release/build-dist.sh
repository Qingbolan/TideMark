#!/usr/bin/env bash
# TideMark
# ========
#
# File: scripts/release/build-dist.sh
# Description: Build release tarball and checksum artifacts for TideMark binaries.
#
# Responsibility:
# - Compile binaries and package distributable archives for release publishing.
#
# Architectural Position:
# - Release artifact assembly helper in the delivery pipeline.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

VERSION="${1:-$(git describe --tags --always --dirty)}"
TARGET="${2:-}"

BUILD_ARGS=(--release)
if [[ -n "$TARGET" ]]; then
  BUILD_ARGS+=(--target "$TARGET")
fi

cargo build "${BUILD_ARGS[@]}" --bin tide --bin git-tide

if [[ -n "$TARGET" ]]; then
  BIN_DIR="target/$TARGET/release"
  TARGET_LABEL="$TARGET"
else
  BIN_DIR="target/release"
  TARGET_LABEL="$(rustc -vV | awk '/host:/ {print $2}')"
fi

DIST_DIR="dist"
PACKAGE_BASE="tidemark-${VERSION}-${TARGET_LABEL}"
PACKAGE_DIR="$DIST_DIR/$PACKAGE_BASE"

rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR"
cp "$BIN_DIR/tide" "$PACKAGE_DIR/tide"
cp "$BIN_DIR/git-tide" "$PACKAGE_DIR/git-tide"
cp README.md "$PACKAGE_DIR/README.md"
cp LICENSE "$PACKAGE_DIR/LICENSE"

mkdir -p "$DIST_DIR"
tar -C "$DIST_DIR" -czf "$DIST_DIR/${PACKAGE_BASE}.tar.gz" "$PACKAGE_BASE"
shasum -a 256 "$DIST_DIR/${PACKAGE_BASE}.tar.gz" > "$DIST_DIR/${PACKAGE_BASE}.tar.gz.sha256"

echo "dist_archive=$DIST_DIR/${PACKAGE_BASE}.tar.gz"
echo "dist_checksum=$DIST_DIR/${PACKAGE_BASE}.tar.gz.sha256"
