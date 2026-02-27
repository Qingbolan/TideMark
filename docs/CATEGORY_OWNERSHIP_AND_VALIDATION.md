<!--
TideMark
========

File: docs/CATEGORY_OWNERSHIP_AND_VALIDATION.md
Description: Category benchmark map, moat thesis, novelty claims, and time-bound validation plan for TideMark.

Responsibility:
- Translate positioning claims into falsifiable structural and evaluation benchmarks.

Architectural Position:
- Strategy-level source of truth linking product narrative to measurable engineering outcomes.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# Category Ownership and Validation

## 1. Category Map (Cognitive / Structural / Evaluation Benchmark)

### Cognitive Benchmark
- Historical anchor: Docker (`immutable identity -> portable trust`).
- TideMark mapping: `Git Version Truth Layer` (`deterministic release identity from Git state`).
- Fit score: `4/5`.

### Structural Benchmark
- Baseline: `git describe + ad-hoc CI scripts + convention-based tagging`.
- Missing invariant: full-path determinism under identical repository input, including remote tag drift, same-timestamp commits, and timezone policy.
- TideMark structural layer:
  - Total-order anchor selection.
  - Day-delta and same-day commit-index state machine.
  - Remote refresh with explicit local fallback semantics.
  - Typed error exits for stable automation handling.
- Evidence:
  - [TECHNICAL_DESIGN.md](/Users/macbook.silan.tech/Documents/GitHub/TideMark/docs/TECHNICAL_DESIGN.md)
  - [release.rs](/Users/macbook.silan.tech/Documents/GitHub/TideMark/src/core/release.rs)

### Evaluation Benchmark
- Observable I/O: `Git history + config -> x.y.z(.tag) + explain key=value`.
- Baseline comparison: `git describe` or manually managed version strings.
- Stable failure surface: `NoReleaseAnchor`, `TimestampAnomaly`, `ConfigParse`, and typed exit codes.
- Evidence snapshot:
  - On February 27, 2026, local `cargo test` passed `22` tests including determinism, remote refresh, and plugin parity checks.

## 2. VC Thesis Card
- Category Size: `3/5`.
- Wedge Sharpness: `4/5`.
- Distribution Loop: `3/5`.
- Structural Moat: `3/5`.
- Six-month copy test:
  - Defensible only if TideMark controls benchmark assets: normative spec + conformance corpus + CI compatibility matrix.
- Risks:
  - Limited public adoption evidence today.
  - Commercial binding scenarios are not yet demonstrated.

## 3. Engineering Truth Card
- Semantic Closure: `4/5`.
- Boundary Clarity: `5/5`.
- Recoverability: `4/5`.
- Boundedness: `5/5`.
- Observability: `4/5`.

### Major Claim 1: Deterministic coordinates
- Invariant: same Git state + same config -> byte-identical output.
- Recovery path: remote lookup failure degrades to local-only when configured, or fails explicitly.
- Evidence: [remote_refresh_integration.rs](/Users/macbook.silan.tech/Documents/GitHub/TideMark/tests/remote_refresh_integration.rs)

### Major Claim 2: Side-effect free core semantics
- Invariant: no commit/tag/index/worktree mutation in resolver flow.
- Recovery path: unmet preconditions map to stable typed exits.
- Evidence: [error.rs](/Users/macbook.silan.tech/Documents/GitHub/TideMark/src/error.rs)

### Major Claim 3: Auditable outputs
- Invariant: explain output field set remains stable and script-safe.
- Recovery path: invalid config/timezone is rejected with explicit diagnostics.

### Current Gaps
- No published performance upper-bound baseline.
- No extreme history-graph benchmark report yet.

## 4. Brand Compression Mapping
- `Git-native deterministic version coordinate CLI`
  - Contact surface: `tide mark`, `tide file`, `git tide mark`.
  - Primitive: total-order anchor selection + day delta + commit index.
  - Metric: repeated-run byte-consistency rate.
  - Consequence: eligible as a release gate, not only a display helper.
- `Release truth from Git state only`
  - Contact surface: `--local-only` and remote refresh policy.
  - Primitive: `RemoteStrategy`, `fallback_to_local`, typed exits.
  - Metric: convergence after remote tag updates and explainability rate.
  - Consequence: lower release dispute and rollback coordination cost.
- `Automation-friendly version protocol layer`
  - Contact surface: service plan/install and explain key-value output.
  - Primitive: script-safe formatter + systemd timer planning.
  - Metric: CI integration lead time and failure triage latency.
  - Consequence: higher delivery throughput.
- Rejected claim: `next-generation version intelligence engine`.
  - Reason: no concrete contact surface, primitive, or measurable metric.

## 5. Monopoly Lane Definition
- Cognitive monopoly statement:
  - TideMark seeks to own `Git Version Truth Layer`, analogous to Docker digest identity for containers.
  - Falsifier: if TideMark is not used as a release gate in at least 20 public repositories by December 31, 2026, this claim fails.
- Structural monopoly statement:
  - TideMark owns the protocol layer for provable anchor selection and coordinate generation.
  - Falsifier: if by Q4 2026 there is no public conformance suite (>=100 cases) and no at least two third-party passing implementations, the structural claim fails.

## 6. Top-Conference 3-Novelty Pack

### Novelty 1: Problem / Formalization
- Claim: formalizes version-coordinate determinism under mutable Git tags, remote drift, and timezone policy.
- Baseline: `git describe` plus scripts.
- Null hypothesis: existing tooling is sufficiently deterministic.
- Failure mode: output drift under identical logical repository state.
- Validation plan: cross-timezone, same-second commit, and tag-rewrite corpus with repeated-run consistency checks.

### Novelty 2: Mechanism / System
- Claim: combines total-order anchor selection, same-day commit indexing, remote same-name override, and cache-bypass conditions.
- Baseline: nearest-tag plus commit-count style schemes.
- Null hypothesis: the mechanism yields no meaningful robustness gain.
- Failure mode: stale output after remote tag updates.
- Validation plan: remote tag redefinition regression with cache-hit vs bypass control experiments.

### Novelty 3: Evaluation / Audit
- Claim: introduces a triad audit frame of explain surface + typed exits + integration regression.
- Baseline: plain string-only version output.
- Null hypothesis: audit surface does not improve operability.
- Failure mode: failures cannot be attributed quickly.
- Validation plan: measure MTTR, error classification precision, and parity (`tide` vs `git-tide`).

## 7. 30/60/90 Validation Plan
- 30 days:
  - Publish category definition v0.1 and concise positioning rewrite.
  - Build at least 30 determinism and edge-case conformance tests.
  - Publish comparison report versus `git describe`.
- 60 days:
  - Release conformance suite v1 with at least 60 cases.
  - Integrate into at least 5 real repository CI pipelines.
  - Publish failure-mode distribution statistics by typed error class.
- 90 days:
  - Reach at least 20 repository trials.
  - Enable at least 2 external adapters or independent implementations.
  - Ship the benchmark trio: specification, conformance corpus, compatibility matrix.
- Primary risk:
  - External adoption and ecosystem partner availability remain unproven and must be validated first.

## 8. Go / No-Go Verdict and Confidence
- Verdict: `Go (conditional)`.
- Condition:
  - Within 90 days, convert engineering correctness into category benchmark control through specification, corpus, and ecosystem compatibility artifacts.
- Confidence: `78%`.
