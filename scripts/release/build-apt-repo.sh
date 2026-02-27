#!/usr/bin/env bash
# TideMark
# ========
#
# File: scripts/release/build-apt-repo.sh
# Description: Build a static APT repository directory from a Debian package artifact.
#
# Responsibility:
# - Generate `pool/` and `dists/` metadata trees for static hosting.
# - Optionally sign Release metadata when GPG inputs are provided.
#
# Architectural Position:
# - Release repository assembly helper for APT distribution publishing.
#
# Author: Silan.Hu
# Email: silan.hu@u.nus.edu
# Copyright (c) 2026-2027 easynet. All rights reserved.

set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "usage: $0 <deb-file> <output-dir> [distribution] [component]" >&2
  exit 2
fi

DEB_FILE="$1"
OUTPUT_DIR="$2"
DISTRIBUTION="${3:-stable}"
COMPONENT="${4:-main}"

if [[ ! -f "$DEB_FILE" ]]; then
  echo "error: deb file not found: $DEB_FILE" >&2
  exit 3
fi

if ! command -v dpkg-deb >/dev/null 2>&1; then
  echo "error: dpkg-deb is required" >&2
  exit 4
fi

if ! command -v dpkg-scanpackages >/dev/null 2>&1; then
  echo "error: dpkg-scanpackages is required" >&2
  exit 5
fi

if ! command -v apt-ftparchive >/dev/null 2>&1; then
  echo "error: apt-ftparchive is required" >&2
  exit 6
fi

ARCH="$(dpkg-deb --field "$DEB_FILE" Architecture)"
if [[ -z "$ARCH" ]]; then
  echo "error: failed to resolve architecture from $DEB_FILE" >&2
  exit 7
fi

REPO_ROOT="$(mktemp -d)"
trap 'rm -rf "$REPO_ROOT"' EXIT

POOL_DIR="$REPO_ROOT/pool/$COMPONENT"
DIST_DIR="$REPO_ROOT/dists/$DISTRIBUTION/$COMPONENT/binary-$ARCH"

mkdir -p "$POOL_DIR" "$DIST_DIR"
cp "$DEB_FILE" "$POOL_DIR/"

(
  cd "$REPO_ROOT"
  dpkg-scanpackages --arch "$ARCH" "pool/$COMPONENT" /dev/null > "$DIST_DIR/Packages"
)
gzip -9c "$DIST_DIR/Packages" > "$DIST_DIR/Packages.gz"

(
  cd "$REPO_ROOT"
  apt-ftparchive release "dists/$DISTRIBUTION" > "dists/$DISTRIBUTION/Release"
)

if [[ -n "${APT_GPG_KEY_ID:-}" ]]; then
  if ! command -v gpg >/dev/null 2>&1; then
    echo "error: gpg is required when APT_GPG_KEY_ID is set" >&2
    exit 8
  fi
  GPG_ARGS=(--batch --yes --pinentry-mode loopback)
  if [[ -n "${APT_GPG_PASSPHRASE:-}" ]]; then
    GPG_ARGS+=(--passphrase "$APT_GPG_PASSPHRASE")
  fi
  gpg "${GPG_ARGS[@]}" --default-key "$APT_GPG_KEY_ID" --detach-sign \
    --armor --output "$REPO_ROOT/dists/$DISTRIBUTION/Release.gpg" \
    "$REPO_ROOT/dists/$DISTRIBUTION/Release"
  gpg "${GPG_ARGS[@]}" --default-key "$APT_GPG_KEY_ID" --clearsign \
    --output "$REPO_ROOT/dists/$DISTRIBUTION/InRelease" \
    "$REPO_ROOT/dists/$DISTRIBUTION/Release"
fi

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
cp -R "$REPO_ROOT/." "$OUTPUT_DIR/"

echo "apt_distribution=$DISTRIBUTION"
echo "apt_component=$COMPONENT"
echo "apt_architecture=$ARCH"
echo "apt_repo_dir=$OUTPUT_DIR"
