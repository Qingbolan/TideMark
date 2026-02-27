//! TideMark
//! ========
//!
//! File: src/infra/git/mod.rs
//! Description: Git provider trait that abstracts repository queries required by TideMark core algorithms.
//!
//! Responsibility:
//! - Define the capability contract for commit, ancestry, tag, and file-history lookups.
//!
//! Architectural Position:
//! - Infrastructure abstraction boundary between core semantics and concrete Git backends.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

pub mod cli;

use std::path::{Path, PathBuf};

use crate::{
    core::model::{CommitInfo, TagRef},
    error::TideResult,
};

pub trait GitProvider {
    fn repo_root(&self) -> &Path;
    fn git_dir(&self) -> TideResult<PathBuf>;
    fn head_commit(&self) -> TideResult<CommitInfo>;
    fn resolve_commit(&self, rev: &str) -> TideResult<CommitInfo>;
    fn commit_exists(&self, rev: &str) -> TideResult<bool>;
    fn list_local_tags(&self, prefix: &str) -> TideResult<Vec<TagRef>>;
    fn list_remote_tags(&self, remote: &str, prefix: &str) -> TideResult<Vec<TagRef>>;
    fn is_ancestor(&self, ancestor: &str, descendant: &str) -> TideResult<bool>;
    fn commit_distance(&self, ancestor: &str, descendant: &str) -> TideResult<u32>;
    fn ancestry_path_commits(
        &self,
        ancestor: &str,
        descendant: &str,
    ) -> TideResult<Vec<CommitInfo>>;
    fn last_modifying_commit(&self, path: &Path, follow_renames: bool) -> TideResult<CommitInfo>;
    fn current_branch(&self) -> TideResult<Option<String>>;
    fn root_commit(&self) -> TideResult<CommitInfo>;
}
