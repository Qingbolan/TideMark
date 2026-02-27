"""
TideMark
========

File: packaging/pypi/tidemark_agent/cli.py
Description: Python console-script entrypoints for TideMark launcher commands.

Responsibility:
- Dispatch `tide` and `git-tide` invocations to resolved native binaries.

Architectural Position:
- Python CLI adapter boundary for package-manager users.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
"""

from __future__ import annotations

import sys

from .resolver import run_binary


def tide_main() -> None:
    raise SystemExit(run_binary("tide", sys.argv[1:]))


def git_tide_main() -> None:
    raise SystemExit(run_binary("git-tide", sys.argv[1:]))
