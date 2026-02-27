<!--
TideMark
========

File: docs/DELIVERY_REQUIREMENTS.md
Description: Delivery scope and acceptance requirements for TideMark operations, distribution, and plugin behavior.

Responsibility:
- Define what must be delivered and how completion is evaluated.

Architectural Position:
- Program-level requirement baseline for release readiness.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# TideMark Delivery Scope and Requirements

## 1. Goals
- Preserve deterministic and traceable version-coordinate resolution.
- Add operations support through user-level systemd timer registration.
- Add distribution support for Homebrew and APT paths.
- Add Git plugin support through `git tide ...`.

## 2. Acceptance Criteria
1. Plugin capability
- After installation, `git tide mark` must execute successfully.
- `git-tide mark` and `tide mark` must produce identical output in the same repository.

2. Service capability
- `tide service plan` must print deterministic `.service` and `.timer` content.
- `tide service install` on Linux must install user units and run `systemctl --user enable --now`.
- `tide service uninstall` must disable and remove the corresponding units.

3. Release capability
- Build cross-platform binary tarballs containing both `tide` and `git-tide`.
- Build `.deb` artifacts.
- Provide Homebrew formula template and GitHub Release workflow.

## 3. Boundaries and Assumptions
- Service registration is Linux + systemd user-session only.
- No self-modifying binary `self-update` flow is implemented.
- APT publishing relies on external repository tooling (for example `aptly`); this repository only ships scripts and process guidance.
