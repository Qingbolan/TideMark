"""
TideMark
========

File: packaging/pypi/tidemark_agent/__init__.py
Description: Python launcher package for TideMark command wrappers.

Responsibility:
- Expose package version metadata for runtime binary resolution.

Architectural Position:
- Python distribution runtime boundary for command entrypoints.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
"""

from importlib.metadata import version


def package_version() -> str:
    return version("tidemark")
