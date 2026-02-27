//! TideMark
//! ========
//!
//! File: src/core/resolver/mark.rs
//! Description: Commit-to-coordinate resolver for `x.y.z(.tag)` generation.
//!
//! Responsibility:
//! - Resolve anchor, day delta, and same-day commit index with deterministic tie-breaking.
//!
//! Architectural Position:
//! - Core algorithm layer for mark resolution and explain payload generation.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use serde::Serialize;

use crate::{
    config::{RemoteStrategy, TideConfig},
    core::{
        model::{
            AnchorSelection, CommitInfo, MarkExplain, MarkResult, ReleaseTag, TagRef, TagSource,
            VersionCoordinate,
        },
        release,
        time::TimezonePolicy,
    },
    error::{TideError, TideResult},
    infra::{cache::CacheStore, git::GitProvider},
};

const MARK_CACHE_NAMESPACE: &str = "mark";

#[derive(Debug, Clone)]
pub struct MarkRequest {
    pub target_rev: Option<String>,
    pub local_only: bool,
    pub metadata_suffix: Option<String>,
}

#[derive(Debug, Serialize)]
struct MarkCacheKey<'a> {
    target_commit: &'a str,
    local_only: bool,
    tag_prefix: &'a str,
    require_annotated_tags: bool,
    timezone: &'a str,
    remote_strategy: &'a str,
    remote_name: &'a str,
    metadata_suffix: Option<&'a str>,
}

pub fn resolve_mark(
    git: &dyn GitProvider,
    config: &TideConfig,
    cache: &CacheStore,
    req: MarkRequest,
) -> TideResult<MarkResult> {
    let timezone = TimezonePolicy::parse(config.time.timezone.as_str())?;
    let target = match req.target_rev.as_deref() {
        Some(rev) => git.resolve_commit(rev)?,
        None => git.head_commit()?,
    };

    let metadata = normalize_metadata_suffix(req.metadata_suffix, &config.output.metadata_suffix);
    let bypass_cache = requires_remote_refresh(config, req.local_only);

    let mut cache_key = None;
    if !bypass_cache {
        let key_payload = MarkCacheKey {
            target_commit: target.id.as_str(),
            local_only: req.local_only,
            tag_prefix: config.release.tag_prefix.as_str(),
            require_annotated_tags: config.release.require_annotated_tags,
            timezone: config.time.timezone.as_str(),
            remote_strategy: remote_strategy_label(&config.remote.strategy),
            remote_name: config.remote.name.as_str(),
            metadata_suffix: metadata.as_deref(),
        };
        let resolved_key = CacheStore::key_from_serializable(MARK_CACHE_NAMESPACE, &key_payload)?;
        if let Some(cached) =
            cache.get::<MarkResult>(MARK_CACHE_NAMESPACE, resolved_key.as_str())?
        {
            return Ok(cached);
        }
        cache_key = Some(resolved_key);
    }

    let (releases, remote_status) = release::load_release_tags(git, config, req.local_only)?;
    let anchor = match release::select_anchor(
        git,
        releases.as_slice(),
        &target,
        config.release.tag_prefix.as_str(),
    ) {
        Ok(a) => a,
        Err(TideError::NoReleaseAnchor { .. }) => {
            let root = git.root_commit()?;
            let distance = git.commit_distance(root.id.as_str(), target.id.as_str())?;
            AnchorSelection {
                release: ReleaseTag {
                    anchor_value: 0,
                    tag: TagRef {
                        name: "(none)".to_string(),
                        commit_id: root.id.clone(),
                        is_annotated: false,
                        source: TagSource::Local,
                    },
                },
                distance,
                anchor_commit: root,
            }
        }
        Err(other) => return Err(other),
    };

    let day_delta_i64 = timezone.day_delta(anchor.anchor_commit.timestamp, target.timestamp)?;
    if day_delta_i64 < 0 {
        return Err(TideError::TimestampAnomaly {
            anchor_ts: anchor.anchor_commit.timestamp,
            target_ts: target.timestamp,
        });
    }
    let day_delta = u32::try_from(day_delta_i64).map_err(|_| TideError::Internal {
        message: format!("day delta overflow: {day_delta_i64}"),
    })?;

    let commit_index = resolve_commit_index(git, &timezone, &anchor.anchor_commit, &target)?;

    let coordinate = VersionCoordinate {
        x: anchor.release.anchor_value,
        y: day_delta,
        z: commit_index,
        metadata,
    };

    let explain = MarkExplain {
        version: coordinate.clone(),
        target_commit: target,
        anchor_tag: anchor.release.tag.name,
        anchor_commit: anchor.anchor_commit,
        day_delta,
        commit_index,
        timezone: timezone.canonical_name(),
        remote_status,
        branch: git.current_branch()?,
    };

    let result = MarkResult {
        coordinate,
        explain,
    };
    if let Some(key) = cache_key.as_deref() {
        cache.put(MARK_CACHE_NAMESPACE, key, &result)?;
    }

    Ok(result)
}

fn normalize_metadata_suffix(
    arg_suffix: Option<String>,
    config_suffix: &Option<String>,
) -> Option<String> {
    let value = arg_suffix.or_else(|| config_suffix.clone())?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn remote_strategy_label(strategy: &RemoteStrategy) -> &'static str {
    match strategy {
        RemoteStrategy::LsRemote => "ls-remote",
        RemoteStrategy::LocalOnly => "local-only",
    }
}

fn requires_remote_refresh(config: &TideConfig, local_only: bool) -> bool {
    !local_only && config.remote.strategy == RemoteStrategy::LsRemote
}

fn resolve_commit_index(
    git: &dyn GitProvider,
    timezone: &TimezonePolicy,
    anchor_commit: &CommitInfo,
    target: &CommitInfo,
) -> TideResult<u32> {
    if anchor_commit.id == target.id {
        return Ok(0);
    }

    let commits = git.ancestry_path_commits(anchor_commit.id.as_str(), target.id.as_str())?;
    commit_index_on_day(commits.as_slice(), target, timezone)
}

fn commit_index_on_day(
    path_commits: &[CommitInfo],
    target: &CommitInfo,
    timezone: &TimezonePolicy,
) -> TideResult<u32> {
    let target_day = timezone.date_for_timestamp(target.timestamp)?;

    let mut commits_on_day: Vec<CommitInfo> = path_commits
        .iter()
        .filter(|commit| timezone.date_for_timestamp(commit.timestamp).ok() == Some(target_day))
        .cloned()
        .collect();

    commits_on_day.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then_with(|| a.id.cmp(&b.id)));

    let idx = commits_on_day
        .iter()
        .position(|commit| commit.id == target.id)
        .ok_or_else(|| TideError::Internal {
            message: format!(
                "target commit {} not found in ancestry path day partition",
                target.id
            ),
        })?;

    Ok((idx + 1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_index_uses_timestamp_then_hash() {
        let tz = TimezonePolicy::parse("UTC").unwrap();
        let target = CommitInfo {
            id: "b".to_string(),
            timestamp: 100,
        };
        let commits = vec![
            CommitInfo {
                id: "c".to_string(),
                timestamp: 100,
            },
            CommitInfo {
                id: "b".to_string(),
                timestamp: 100,
            },
            CommitInfo {
                id: "a".to_string(),
                timestamp: 100,
            },
        ];

        let idx = commit_index_on_day(commits.as_slice(), &target, &tz).unwrap();
        assert_eq!(idx, 2);
    }
}
