/*
TideMark
========

File: packaging/npm/lib/runner.js
Description: Runtime binary resolver and launcher for npm TideMark wrappers.

Responsibility:
- Resolve binary cache path, download verified archives, and execute native commands.

Architectural Position:
- Node runtime bridge from package manager entrypoints to release binaries.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
*/

const { createHash } = require("node:crypto");
const fs = require("node:fs/promises");
const { existsSync } = require("node:fs");
const { homedir } = require("node:os");
const path = require("node:path");
const { spawn } = require("node:child_process");
const https = require("node:https");
const tar = require("tar");

const { resolveTargetTriple } = require("./platform");

const PACKAGE_JSON = require("../package.json");
const DEFAULT_REPOSITORY = PACKAGE_JSON.tidemarkRepository || "Qingbolan/TideMark";

function resolveRepository() {
  return process.env.TIDEMARK_GITHUB_REPOSITORY || DEFAULT_REPOSITORY;
}

function resolveCacheRoot() {
  return process.env.TIDEMARK_CACHE_DIR || path.join(homedir(), ".cache", "tidemark-npm");
}

function releaseAssetUrl(version, targetTriple) {
  const repository = resolveRepository();
  return `https://github.com/${repository}/releases/download/v${version}/tidemark-${version}-${targetTriple}.tar.gz`;
}

function download(url) {
  return new Promise((resolve, reject) => {
    https
      .get(url, (response) => {
        if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          return resolve(download(response.headers.location));
        }
        if (response.statusCode !== 200) {
          return reject(new Error(`download failed: ${url}, status=${response.statusCode}`));
        }
        const chunks = [];
        response.on("data", (chunk) => chunks.push(chunk));
        response.on("end", () => resolve(Buffer.concat(chunks)));
      })
      .on("error", reject);
  });
}

function verifyChecksum(archiveBytes, checksumText) {
  const expected = checksumText.trim().split(/\s+/)[0];
  const actual = createHash("sha256").update(archiveBytes).digest("hex");
  if (actual !== expected) {
    throw new Error(`checksum mismatch: expected=${expected}, actual=${actual}`);
  }
}

async function writeArchive(tempDir, archiveBytes) {
  const archivePath = path.join(tempDir, "artifact.tar.gz");
  await fs.writeFile(archivePath, archiveBytes);
  await tar.extract({
    cwd: tempDir,
    file: archivePath,
    gzip: true,
  });
  return archivePath;
}

async function ensureInstalled() {
  const version = PACKAGE_JSON.version;
  const targetTriple = resolveTargetTriple();
  const cacheRoot = resolveCacheRoot();
  await fs.mkdir(cacheRoot, { recursive: true });
  const installDir = path.join(cacheRoot, version, targetTriple);
  const tidePath = path.join(installDir, "tide");
  const gitTidePath = path.join(installDir, "git-tide");

  if (existsSync(tidePath) && existsSync(gitTidePath)) {
    return installDir;
  }

  const archiveUrl = releaseAssetUrl(version, targetTriple);
  const archiveBytes = await download(archiveUrl);
  const checksumBytes = await download(`${archiveUrl}.sha256`);
  verifyChecksum(archiveBytes, checksumBytes.toString("utf-8"));

  const tempDir = await fs.mkdtemp(path.join(cacheRoot, "tmp-"));
  try {
    await writeArchive(tempDir, archiveBytes);
    const extractedDir = path.join(tempDir, `tidemark-${version}-${targetTriple}`);
    await fs.mkdir(installDir, { recursive: true });
    await fs.copyFile(path.join(extractedDir, "tide"), tidePath);
    await fs.copyFile(path.join(extractedDir, "git-tide"), gitTidePath);
    await fs.chmod(tidePath, 0o755);
    await fs.chmod(gitTidePath, 0o755);
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }

  return installDir;
}

async function runBinary(binaryName, args) {
  const installDir = await ensureInstalled();
  const binaryPath = path.join(installDir, binaryName);
  await new Promise((resolve, reject) => {
    const child = spawn(binaryPath, args, { stdio: "inherit" });
    child.on("error", reject);
    child.on("close", (code) => resolve(code));
  }).then((code) => {
    process.exitCode = code === null ? 1 : code;
  });
}

async function prefetchBinaries() {
  await ensureInstalled();
}

module.exports = {
  prefetchBinaries,
  runBinary,
};
