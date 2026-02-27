<!--
TideMark
========

File: PROJECT_STRUCTURE.md
Description: Repository structure contract and layering invariants for TideMark.

Responsibility:
- Define directory semantics and migration rules for architectural consistency.

Architectural Position:
- Source-of-truth document for repository organization.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Project Structure Contract

This file defines TideMark's repository structure contract, aligned with the Axon-style layered philosophy.

## Semantic Layers

1. `src/core`: deterministic versioning domain logic only.
2. `src/infra`: infrastructure adapters (git/cache) and external process boundaries.
3. `src/interface`: input/output boundaries (CLI argument model and output rendering).
4. `src/app`: application orchestration that wires interface + infra + core.
5. `src/ops`: operational capabilities (service install/plan/uninstall).
6. `docs`, `tests`, `packaging`, `scripts`: documentation, verification, and delivery assets.

## Invariants

1. `core` must not depend on `interface` or `ops`.
2. `interface` should format/parse only; no version calculation logic.
3. `infra` owns all git/system side effects; `core` remains deterministic over provided data.
4. `app` is a thin facade that composes modules but does not duplicate domain algorithms.
5. One file should keep one dominant responsibility.

## Migration Rules

1. Physical directory moves should be done before behavior changes.
2. Keep docs and module paths updated in the same change set when moving files.
3. Do not introduce cross-layer back references.
