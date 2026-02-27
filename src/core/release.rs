//! TideMark
//! ========
//!
//! File: src/core/release.rs
//! Description: Release-tag parsing, loading, merge policy, and anchor-selection logic.
//!
//! Responsibility:
//! - Select a deterministic anchor commit from eligible release tags based on explicit ordering rules.
//!
//! Architectural Position:
//! - Core release semantics layer that bridges Git tag inventories into anchor candidates.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{cmp::Ordering, collections::BTreeMap};

use crate::{
    config::{RemoteStrategy, TideConfig},
    core::model::{AnchorSelection, CommitInfo, ReleaseTag, RemoteLoadStatus, TagRef},
    error::{TideError, TideResult},
    infra::git::GitProvider,
};

/// Extract a sortable semver key from a tag name (e.g. "v0.1.0" → (0,1,0), "v3" → (3,0,0)).
/// Used only for ordering tags; the final `anchor_value` is the 1-based ordinal position.
pub fn parse_sort_key(tag_name: &str, prefix: &str) -> TideResult<(u64, u64, u64)> {
    let suffix = tag_name
        .strip_prefix(prefix)
        .ok_or_else(|| TideError::InvalidReleaseTag {
            tag: tag_name.to_string(),
            prefix: prefix.to_string(),
        })?;

    let parts: Vec<&str> = suffix.splitn(3, '.').collect();
    let parse_part = |s: &str| -> Option<u64> {
        let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            None
        } else {
            digits.parse::<u64>().ok()
        }
    };

    let major = parse_part(parts.first().unwrap_or(&""))
        .ok_or_else(|| TideError::InvalidReleaseTag {
            tag: tag_name.to_string(),
            prefix: prefix.to_string(),
        })?;
    let minor = parts.get(1).and_then(|s| parse_part(s)).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| parse_part(s)).unwrap_or(0);

    Ok((major, minor, patch))
}

pub fn load_release_tags(
    git: &dyn GitProvider,
    config: &TideConfig,
    local_only: bool,
) -> TideResult<(Vec<ReleaseTag>, RemoteLoadStatus)> {
    let mut by_name: BTreeMap<String, TagRef> = BTreeMap::new();
    for tag in git.list_local_tags(config.release.tag_prefix.as_str())? {
        by_name.insert(tag.name.clone(), tag);
    }

    let mut remote_status = RemoteLoadStatus::NotAttempted;
    let should_attempt_remote = !local_only && config.remote.strategy == RemoteStrategy::LsRemote;
    if should_attempt_remote {
        match git.list_remote_tags(
            config.remote.name.as_str(),
            config.release.tag_prefix.as_str(),
        ) {
            Ok(remote_tags) => {
                remote_status = RemoteLoadStatus::UsedRemote;
                for tag in remote_tags {
                    // In remote mode, same-name remote tags override local tags so the
                    // coordinate reflects the latest remote definition.
                    by_name.insert(tag.name.clone(), tag);
                }
            }
            Err(err) => {
                if config.remote.fallback_to_local {
                    remote_status = RemoteLoadStatus::FallbackLocal;
                    let _ = err;
                } else {
                    return Err(err);
                }
            }
        }
    }

    let mut keyed: Vec<((u64, u64, u64), TagRef)> = Vec::new();
    for tag in by_name.into_values() {
        if config.release.require_annotated_tags && !tag.is_annotated {
            continue;
        }

        let sort_key =
            parse_sort_key(tag.name.as_str(), config.release.tag_prefix.as_str())?;
        keyed.push((sort_key, tag));
    }

    keyed.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.name.cmp(&b.1.name))
    });

    let releases: Vec<ReleaseTag> = keyed
        .into_iter()
        .enumerate()
        .map(|(i, (_key, tag))| ReleaseTag {
            anchor_value: (i + 1) as u64,
            tag,
        })
        .collect();

    Ok((releases, remote_status))
}

pub fn select_anchor(
    git: &dyn GitProvider,
    releases: &[ReleaseTag],
    target: &CommitInfo,
    prefix: &str,
) -> TideResult<AnchorSelection> {
    let mut selected: Option<AnchorSelection> = None;

    for release in releases {
        if !git.commit_exists(release.tag.commit_id.as_str())? {
            continue;
        }

        if !git.is_ancestor(release.tag.commit_id.as_str(), target.id.as_str())? {
            continue;
        }

        let distance = git.commit_distance(release.tag.commit_id.as_str(), target.id.as_str())?;
        let anchor_commit = git.resolve_commit(release.tag.commit_id.as_str())?;
        let candidate = AnchorSelection {
            release: release.clone(),
            distance,
            anchor_commit,
        };

        if is_better_anchor(selected.as_ref(), &candidate) {
            selected = Some(candidate);
        }
    }

    selected.ok_or_else(|| TideError::NoReleaseAnchor {
        prefix: prefix.to_string(),
    })
}

fn is_better_anchor(current: Option<&AnchorSelection>, candidate: &AnchorSelection) -> bool {
    match current {
        None => true,
        Some(existing) => compare_anchor(candidate, existing) == Ordering::Less,
    }
}

fn compare_anchor(left: &AnchorSelection, right: &AnchorSelection) -> Ordering {
    left.distance
        .cmp(&right.distance)
        .then_with(|| right.release.anchor_value.cmp(&left.release.anchor_value))
        .then_with(|| left.release.tag.name.cmp(&right.release.tag.name))
        .then_with(|| left.release.tag.commit_id.cmp(&right.release.tag.commit_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sort_key_from_default_prefix() {
        assert_eq!(parse_sort_key("v1", "v").unwrap(), (1, 0, 0));
        assert_eq!(parse_sort_key("v0.1.0", "v").unwrap(), (0, 1, 0));
        assert_eq!(parse_sort_key("v12.3.4", "v").unwrap(), (12, 3, 4));
        assert!(parse_sort_key("v", "v").is_err());
    }
}
