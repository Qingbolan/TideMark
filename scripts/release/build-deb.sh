#!/usr/bin/env bash
# TideMark
# ========
#
# File: scripts/release/build-deb.sh
# Description: Build Debian package artifacts for TideMark binaries.
#
# Responsibility:
# - Produce `.deb` output using cargo-deb with release binaries.
#
# Architectural Position:
# - Release packaging helper in the delivery toolchain.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if ! command -v cargo-deb >/dev/null 2>&1; then
  cargo install cargo-deb --locked
fi

cargo build --release --bin tide --bin git-tide
mkdir -p dist
cargo deb --no-build --output dist/

ls -1 dist/*.deb
