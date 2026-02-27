"""
TideMark
========

File: packaging/pypi/tidemark_agent/resolver.py
Description: Runtime binary resolver for TideMark launchers shipped through PyPI.

Responsibility:
- Resolve platform target, download and verify release archives, and execute binaries.

Architectural Position:
- Python runtime bridge between package metadata and GitHub release assets.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
"""

from __future__ import annotations

import hashlib
import io
import os
import platform
import subprocess
import tarfile
import tempfile
import urllib.request
from pathlib import Path

from . import package_version

DEFAULT_REPOSITORY = "Qingbolan/TideMark"


def _resolve_target_triple() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower()

    if system == "linux" and machine in {"x86_64", "amd64"}:
        return "x86_64-unknown-linux-gnu"
    if system == "linux" and machine in {"aarch64", "arm64"}:
        return "aarch64-unknown-linux-gnu"
    if system == "darwin" and machine in {"x86_64", "amd64"}:
        return "x86_64-apple-darwin"
    if system == "darwin" and machine in {"aarch64", "arm64"}:
        return "aarch64-apple-darwin"

    raise RuntimeError(f"unsupported platform: system={system}, machine={machine}")


def _repository() -> str:
    return os.environ.get("TIDEMARK_GITHUB_REPOSITORY", DEFAULT_REPOSITORY)


def _cache_root() -> Path:
    raw = os.environ.get("TIDEMARK_CACHE_DIR", "~/.cache/tidemark")
    return Path(raw).expanduser()


def _asset_url(version: str, target: str) -> str:
    repository = _repository()
    return (
        f"https://github.com/{repository}/releases/download/v{version}/"
        f"tidemark-{version}-{target}.tar.gz"
    )


def _download(url: str) -> bytes:
    with urllib.request.urlopen(url, timeout=60) as response:
        return response.read()


def _verify_checksum(archive_bytes: bytes, checksum_text: str) -> None:
    expected = checksum_text.strip().split()[0]
    actual = hashlib.sha256(archive_bytes).hexdigest()
    if actual != expected:
        raise RuntimeError(
            "checksum mismatch for release archive: "
            f"expected={expected}, actual={actual}"
        )


def _extract_binaries(archive_bytes: bytes, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    expected_names = {"tide", "git-tide"}
    extracted = set()

    with tarfile.open(fileobj=io.BytesIO(archive_bytes), mode="r:gz") as archive:
        for member in archive.getmembers():
            basename = Path(member.name).name
            if basename not in expected_names or not member.isfile():
                continue
            source = archive.extractfile(member)
            if source is None:
                continue
            data = source.read()
            target = destination / basename
            target.write_bytes(data)
            target.chmod(0o755)
            extracted.add(basename)

    missing = expected_names - extracted
    if missing:
        joined = ",".join(sorted(missing))
        raise RuntimeError(f"release archive missing binaries: {joined}")


def _ensure_installed() -> Path:
    version = package_version()
    target = _resolve_target_triple()
    install_dir = _cache_root() / version / target
    tide_binary = install_dir / "tide"
    git_tide_binary = install_dir / "git-tide"
    if tide_binary.exists() and git_tide_binary.exists():
        return install_dir

    archive_url = _asset_url(version, target)
    archive_bytes = _download(archive_url)
    checksum_bytes = _download(f"{archive_url}.sha256")
    _verify_checksum(archive_bytes, checksum_bytes.decode("utf-8"))

    with tempfile.TemporaryDirectory(prefix="tidemark-") as temp_dir:
        temp_install = Path(temp_dir) / "install"
        _extract_binaries(archive_bytes, temp_install)
        install_dir.mkdir(parents=True, exist_ok=True)
        for binary_name in ("tide", "git-tide"):
            target_path = install_dir / binary_name
            target_path.write_bytes((temp_install / binary_name).read_bytes())
            target_path.chmod(0o755)

    return install_dir


def run_binary(binary_name: str, args: list[str]) -> int:
    install_dir = _ensure_installed()
    binary_path = install_dir / binary_name
    if not binary_path.exists():
        raise RuntimeError(f"resolved binary not found: {binary_path}")
    completed = subprocess.run([str(binary_path), *args], check=False)
    return completed.returncode
