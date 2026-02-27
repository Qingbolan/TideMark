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

pub fn parse_anchor_value(tag_name: &str, prefix: &str) -> TideResult<u64> {
    let suffix = tag_name
        .strip_prefix(prefix)
        .ok_or_else(|| TideError::InvalidReleaseTag {
            tag: tag_name.to_string(),
            prefix: prefix.to_string(),
        })?;

    let digits: String = suffix.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(TideError::InvalidReleaseTag {
            tag: tag_name.to_string(),
            prefix: prefix.to_string(),
        });
    }

    digits
        .parse::<u64>()
        .map_err(|_| TideError::InvalidReleaseTag {
            tag: tag_name.to_string(),
            prefix: prefix.to_string(),
        })
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

    let mut releases = Vec::new();
    for tag in by_name.into_values() {
        if config.release.require_annotated_tags && !tag.is_annotated {
            continue;
        }

        let anchor_value =
            parse_anchor_value(tag.name.as_str(), config.release.tag_prefix.as_str())?;
        releases.push(ReleaseTag { anchor_value, tag });
    }

    releases.sort_by(|a, b| {
        a.anchor_value
            .cmp(&b.anchor_value)
            .then_with(|| a.tag.name.cmp(&b.tag.name))
    });

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
    fn parse_anchor_from_default_prefix() {
        assert_eq!(parse_anchor_value("v1", "v").unwrap(), 1);
        assert_eq!(parse_anchor_value("v12.3", "v").unwrap(), 12);
        assert!(parse_anchor_value("v", "v").is_err());
    }
}
