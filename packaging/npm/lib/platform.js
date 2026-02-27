/*
TideMark
========

File: packaging/npm/lib/platform.js
Description: Platform mapping helpers for npm TideMark launchers.

Responsibility:
- Convert Node runtime platform identifiers into release target triples.

Architectural Position:
- Runtime target-resolution utility for npm packaging.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
*/

function resolveTargetTriple() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "linux" && arch === "x64") {
    return "x86_64-unknown-linux-gnu";
  }
  if (platform === "linux" && arch === "arm64") {
    return "aarch64-unknown-linux-gnu";
  }
  if (platform === "darwin" && arch === "x64") {
    return "x86_64-apple-darwin";
  }
  if (platform === "darwin" && arch === "arm64") {
    return "aarch64-apple-darwin";
  }

  throw new Error(`Unsupported platform: ${platform}/${arch}`);
}

module.exports = {
  resolveTargetTriple,
};
