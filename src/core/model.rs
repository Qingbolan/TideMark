//! TideMark
//! ========
//!
//! File: src/core/model.rs
//! Description: Typed domain model for version coordinates, release tags, and explainable resolution output.
//!
//! Responsibility:
//! - Define stable semantic data structures shared across resolver and output layers.
//!
//! Architectural Position:
//! - Canonical domain type boundary for TideMark coordinate resolution.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionCoordinate {
    pub x: u64,
    pub y: u32,
    pub z: u32,
    pub metadata: Option<String>,
}

impl fmt::Display for VersionCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.metadata {
            Some(meta) if !meta.is_empty() => {
                write!(f, "{}.{}.{}.{}", self.x, self.y, self.z, meta)
            }
            _ => write!(f, "{}.{}.{}", self.x, self.y, self.z),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagSource {
    Local,
    Remote,
}

impl fmt::Display for TagSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Remote => write!(f, "remote"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagRef {
    pub name: String,
    pub commit_id: String,
    pub is_annotated: bool,
    pub source: TagSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseTag {
    pub anchor_value: u64,
    pub tag: TagRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteLoadStatus {
    NotAttempted,
    UsedRemote,
    FallbackLocal,
}

impl fmt::Display for RemoteLoadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAttempted => write!(f, "not-attempted"),
            Self::UsedRemote => write!(f, "used-remote"),
            Self::FallbackLocal => write!(f, "fallback-local"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorSelection {
    pub release: ReleaseTag,
    pub distance: u32,
    pub anchor_commit: CommitInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkExplain {
    pub version: VersionCoordinate,
    pub target_commit: CommitInfo,
    pub anchor_tag: String,
    pub anchor_commit: CommitInfo,
    pub day_delta: u32,
    pub commit_index: u32,
    pub timezone: String,
    pub remote_status: RemoteLoadStatus,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkResult {
    pub coordinate: VersionCoordinate,
    pub explain: MarkExplain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileResult {
    pub path: String,
    pub last_commit: CommitInfo,
    pub mark: MarkResult,
}
