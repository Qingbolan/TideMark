//! TideMark
//! ========
//!
//! File: src/core/resolver/file.rs
//! Description: File-to-coordinate resolver built on last-modifying-commit lookup and mark resolution.
//!
//! Responsibility:
//! - Map a tracked path to its last modifying commit and then compute its version coordinate.
//!
//! Architectural Position:
//! - Core resolver adapter that composes file history lookup with mark semantics.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::path::PathBuf;

use serde::Serialize;

use crate::{
    config::{RemoteStrategy, TideConfig},
    core::{
        model::FileResult,
        resolver::mark::{MarkRequest, resolve_mark},
    },
    error::TideResult,
    infra::{cache::CacheStore, git::GitProvider},
};

const FILE_CACHE_NAMESPACE: &str = "file";

#[derive(Debug, Clone)]
pub struct FileRequest {
    pub path: PathBuf,
    pub local_only: bool,
    pub metadata_suffix: Option<String>,
}

#[derive(Debug, Serialize)]
struct FileCacheKey<'a> {
    head_commit: &'a str,
    path: &'a str,
    local_only: bool,
    metadata_suffix: Option<&'a str>,
    follow_renames: bool,
    timezone: &'a str,
    tag_prefix: &'a str,
    require_annotated_tags: bool,
}

pub fn resolve_file(
    git: &dyn GitProvider,
    config: &TideConfig,
    cache: &CacheStore,
    req: FileRequest,
) -> TideResult<FileResult> {
    let head_commit = git.head_commit()?;
    let normalized_path = if req.path.is_absolute() {
        req.path
            .strip_prefix(git.repo_root())
            .unwrap_or(req.path.as_path())
            .to_path_buf()
    } else {
        req.path.clone()
    };
    let path_text = normalized_path.to_string_lossy().to_string();

    let metadata = req
        .metadata_suffix
        .clone()
        .or_else(|| config.output.metadata_suffix.clone())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    let bypass_cache = requires_remote_refresh(config, req.local_only);

    let mut cache_key = None;
    if !bypass_cache {
        let key_payload = FileCacheKey {
            head_commit: head_commit.id.as_str(),
            path: path_text.as_str(),
            local_only: req.local_only,
            metadata_suffix: metadata.as_deref(),
            follow_renames: config.output.follow_renames,
            timezone: config.time.timezone.as_str(),
            tag_prefix: config.release.tag_prefix.as_str(),
            require_annotated_tags: config.release.require_annotated_tags,
        };

        let resolved_key = CacheStore::key_from_serializable(FILE_CACHE_NAMESPACE, &key_payload)?;
        if let Some(cached) =
            cache.get::<FileResult>(FILE_CACHE_NAMESPACE, resolved_key.as_str())?
        {
            return Ok(cached);
        }
        cache_key = Some(resolved_key);
    }

    let last_commit = git.last_modifying_commit(&normalized_path, config.output.follow_renames)?;
    let mark = resolve_mark(
        git,
        config,
        cache,
        MarkRequest {
            target_rev: Some(last_commit.id.clone()),
            local_only: req.local_only,
            metadata_suffix: metadata,
        },
    )?;

    let result = FileResult {
        path: path_text,
        last_commit,
        mark,
    };
    if let Some(key) = cache_key.as_deref() {
        cache.put(FILE_CACHE_NAMESPACE, key, &result)?;
    }
    Ok(result)
}

fn requires_remote_refresh(config: &TideConfig, local_only: bool) -> bool {
    !local_only && config.remote.strategy == RemoteStrategy::LsRemote
}
