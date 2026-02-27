<!--
TideMark
========

File: docs/TECHNICAL_DESIGN.md
Description: Technical design specification for deterministic TideMark version-coordinate resolution.

Responsibility:
- Define domain rules, algorithms, interfaces, and implementation boundaries.

Architectural Position:
- Canonical technical design reference for maintainers and reviewers.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
-->

# TideMark Technical Design

## 1) Scope and Assumptions
- Scope: deterministic version coordinate resolution from Git state only.
- Mutation boundary: no commits, no tag creation, no index/worktree writes. Cache writes under `.git/tidemark-cache/` are allowed.
- Assumption A1: release tags follow `<prefix><digits...>`; default prefix is `v`.
- Assumption A2: anchor date uses anchor commit timestamp (not tagger timestamp) to keep local/remote behavior symmetric.
- Assumption A3: timezone is `UTC` or fixed offset (`+HH:MM` / `-HH:MM`), never host-local implicit time.

## 2) Data Model
```text
VersionCoordinate = (x, y, z, optional_suffix)
  x: u64 anchor value parsed from nearest release tag
  y: u32 natural day delta between anchor commit date and target commit date
  z: u32 position of target commit among same-day commits after anchor
```

Primary structs:
- `CommitInfo { id: String, timestamp: i64 }`
- `TagRef { name, commit_id, is_annotated, source }`
- `ReleaseTag { anchor_value, tag }`
- `AnchorSelection { release, distance, anchor_commit }`
- `MarkResult { coordinate, explain }`
- `FileResult { path, last_commit, mark }`

## 3) Deterministic Rules
Define target commit `c`.

1. Candidate releases:
- All tags matching `release.tag_prefix`.
- If `require_annotated_tags=true`, reject lightweight tags.
- Optional remote refresh (`git fetch` to `refs/tidemark/remote-tags/*`) merges by tag name, remote wins on collision.

2. Anchor selection:
- Keep tags whose commit is present locally and ancestor of `c`.
- For each tag `t`, distance `d(t) = count(commits in t.commit..c)`.
- Choose minimal tuple:
  - `d(t)` ascending
  - `anchor_value` descending
  - `tag.name` ascending
  - `tag.commit_id` ascending

3. Day delta:
- Convert anchor and target timestamps to local dates in configured timezone.
- `y = date(target) - date(anchor)` in natural days.
- If `y < 0`, fail (`TimestampAnomaly`).

4. Commit index `z`:
- If `c == anchor.commit`, `z = 0`.
- Else collect commits on ancestry path `(anchor, c]`.
- Filter commits with `date(commit) == date(c)`.
- Sort by `(timestamp asc, commit_id asc)`.
- `z = 1 + index_of(c)`.

5. Metadata suffix:
- Optional suffix is appended only in output format `x.y.z.suffix`.
- Suffix does not affect `(x,y,z)` and does not participate in anchor selection.

## 4) Edge Cases
- Shallow clone:
  - Missing ancestors/tags can produce `NoReleaseAnchor`.
  - If remote refresh fails and fallback is enabled, resolver degrades to local tags only.
- Detached HEAD:
  - `mark` still resolves; `branch=detached` only in `--explain` output.
- Missing tags:
  - Fails with `NoReleaseAnchor` (exit code 4).
- Timestamp anomalies:
  - If anchor date > target date in configured timezone, fail deterministically.
- Multiple tags at same commit:
  - Tie resolved by anchor value desc then tag lexicographic order.
- Remote unavailable:
  - If `fallback_to_local=true`, continue with local tags and status `fallback-local`.

## 5) Configuration Schema (`.tidemark.toml`)
```toml
[release]
tag_prefix = "v"
require_annotated_tags = true

[time]
timezone = "UTC" # or +08:00 / -05:30

[remote]
strategy = "ls-remote" # or "local-only"
name = "origin"
fallback_to_local = true

[cache]
enabled = true

[output]
metadata_suffix = "" # optional; empty means none
follow_renames = true
```

## 6) Project Structure
```text
src/
  bin/
    tide.rs               # `tide` entrypoint
    git-tide.rs           # `git-tide` entrypoint (for `git tide ...`)
  lib.rs
  app/
    mod.rs                # command orchestration / application wiring
  core/
    mod.rs
    model.rs              # version/domain typed model
    time.rs               # timezone/date math
    release.rs            # release tag loading + anchor selection
    resolver/
      mod.rs
      mark.rs             # commit->coordinate resolver
      file.rs             # path->last-commit->coordinate resolver
  infra/
    mod.rs
    cache.rs              # .git/tidemark-cache persistence
    git/
      mod.rs              # GitProvider trait
      cli.rs              # Git CLI backend implementation
  interface/
    mod.rs
    cli.rs                # clap command model
    output.rs             # script-safe formatting
  ops/
    mod.rs
    service.rs            # systemd user service planning/install/uninstall
  config.rs               # config schema/load/init
  error.rs                # typed errors + exit-code mapping

docs/
  TECHNICAL_DESIGN.md
  DELIVERY_REQUIREMENTS.md
  DISTRIBUTION_AND_PLUGIN.md

tests/
  common/mod.rs
  mark_integration.rs
  file_integration.rs
  determinism_regression.rs
  plugin_service_integration.rs
```

## 7) Key Signatures (Phase 1)
```rust
pub trait GitProvider {
    fn head_commit(&self) -> TideResult<CommitInfo>;
    fn resolve_commit(&self, rev: &str) -> TideResult<CommitInfo>;
    fn commit_exists(&self, rev: &str) -> TideResult<bool>;
    fn list_local_tags(&self, prefix: &str) -> TideResult<Vec<TagRef>>;
    fn list_remote_tags(&self, remote: &str, prefix: &str) -> TideResult<Vec<TagRef>>;
    fn is_ancestor(&self, ancestor: &str, descendant: &str) -> TideResult<bool>;
    fn commit_distance(&self, ancestor: &str, descendant: &str) -> TideResult<u32>;
    fn ancestry_path_commits(&self, ancestor: &str, descendant: &str) -> TideResult<Vec<CommitInfo>>;
    fn last_modifying_commit(&self, path: &Path, follow_renames: bool) -> TideResult<CommitInfo>;
}

pub fn load_release_tags(
    git: &dyn GitProvider,
    config: &TideConfig,
    local_only: bool,
) -> TideResult<(Vec<ReleaseTag>, RemoteLoadStatus)>;

pub fn select_anchor(
    git: &dyn GitProvider,
    releases: &[ReleaseTag],
    target: &CommitInfo,
    prefix: &str,
) -> TideResult<AnchorSelection>;

pub fn resolve_mark(
    git: &dyn GitProvider,
    config: &TideConfig,
    cache: &CacheStore,
    req: MarkRequest,
) -> TideResult<MarkResult>;

pub fn resolve_file(
    git: &dyn GitProvider,
    config: &TideConfig,
    cache: &CacheStore,
    req: FileRequest,
) -> TideResult<FileResult>;
```

## 8) Algorithm Pseudocode
### `tide mark`
```text
target := HEAD
cfg := load_or_default(.tidemark.toml)
releases, remote_status := load_release_tags(cfg, local_only_flag)
anchor := select_anchor(releases, target)

y := day_delta(anchor.commit.timestamp, target.timestamp, cfg.time.timezone)
if y < 0 -> error TimestampAnomaly

if target == anchor.commit:
  z := 0
else:
  commits := ancestry_path(anchor.commit, target)
  same_day := filter(commits, date(commit.ts) == date(target.ts))
  sort(same_day, by ts asc then id asc)
  z := position(target in same_day) + 1

x := anchor.anchor_value
suffix := cli_suffix or cfg.output.metadata_suffix
emit format "x.y.z(.suffix)"
```

### `tide file <path>`
```text
target := last_modifying_commit(path, follow_renames)
mark := resolve_mark(target_rev = target.id)
emit mark.coordinate
```

## 9) Test Strategy
Unit tests:
- Tag parser (`v1`, `v12.3`, invalid).
- Timezone parser and day-delta behavior.
- Same-day ordering: timestamp tie breaks by commit hash.

Integration tests with fixture repos:
- Build temporary git repo with controlled commit timestamps.
- Annotated tag anchor + mixed-day commits -> expected mark output.
- File resolver maps path to commit-specific coordinate.
- Annotated-tags default enforcement and override via config.

Determinism regression tests:
- Re-run `tide mark` multiple times on same repo/config and assert byte-equal output.
- Same timestamp multi-commit scenario remains stable due `(timestamp, hash)` order.

## 10) Phase Roadmap
Phase 1: Core engine
- CLI backend (`git`), deterministic mark/file, typed errors, config load/init, explain output.

Phase 2: Remote/tag refresh + cache
- Remote refresh strategy with deterministic local fallback and explicit cache-bypass in remote mode.
- Cache key versioning and optional expiry policy.

Phase 3: CI integration hooks
- Stable exit codes and machine-readable explain mode.
- Add `--check` style workflows and release-gating examples.
- Add CI + release workflow (multi-platform binaries + checksums + deb artifact).

Phase 4: Release intelligence extensions
- Optional diagnostics: anchor drift, tag hygiene, shallow clone warnings.
- libgit2 backend implementation behind `GitProvider` parity tests.

## 11) Ops Extensions (Current)
- Git plugin binary:
  - Build `git-tide` beside `tide`; Git can dispatch `git tide ...` through external command discovery.
- Service registration:
  - `tide service plan|install|uninstall` generates deterministic user unit names from repo path hash.
  - Linux-only install/uninstall boundary; non-Linux returns typed `UnsupportedPlatform`.
- Distribution:
  - Release tarball and checksum script.
  - Debian packaging metadata (`cargo-deb`) and publish helper script for APT workflows.
  - Homebrew formula template for release artifacts.
