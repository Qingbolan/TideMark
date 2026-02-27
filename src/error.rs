//! TideMark
//! ========
//!
//! File: src/error.rs
//! Description: Typed error catalog and exit-code mapping for TideMark command execution.
//!
//! Responsibility:
//! - Encode deterministic failure classes across configuration, Git, resolver, and service boundaries.
//!
//! Architectural Position:
//! - Cross-layer error contract consumed by all runtime modules and binaries.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{io, path::PathBuf, process::ExitCode};

use thiserror::Error;

pub type TideResult<T> = Result<T, TideError>;

#[derive(Debug, Error)]
pub enum TideError {
    #[error("not a git repository: {path}")]
    NotGitRepository { path: PathBuf },

    #[error("git command failed: git {args:?}; stderr: {stderr}")]
    GitCommand {
        args: Vec<String>,
        stderr: String,
        code: Option<i32>,
    },

    #[error("invalid UTF-8 from git command")]
    InvalidUtf8,

    #[error("invalid release tag `{tag}` for prefix `{prefix}`")]
    InvalidReleaseTag { tag: String, prefix: String },

    #[error("no release anchor found for prefix `{prefix}`")]
    NoReleaseAnchor { prefix: String },

    #[error(
        "timestamp anomaly: anchor timestamp {anchor_ts} is later than target timestamp {target_ts}"
    )]
    TimestampAnomaly { anchor_ts: i64, target_ts: i64 },

    #[error("invalid timezone value `{value}`; expected `UTC` or +/-HH:MM")]
    InvalidTimezone { value: String },

    #[error("config parse failed at {path}: {message}")]
    ConfigParse { path: PathBuf, message: String },

    #[error("config file already exists at {path}")]
    ConfigExists { path: PathBuf },

    #[error("file has no tracked git history: {path}")]
    FileHistoryNotFound { path: PathBuf },

    #[error("cache format error: {message}")]
    CacheFormat { message: String },

    #[error("service interval must be >= 1 minute; got {minutes}")]
    InvalidServiceInterval { minutes: u32 },

    #[error("feature not supported on this platform: {feature}")]
    UnsupportedPlatform { feature: String },

    #[error("HOME environment variable is not set")]
    MissingHomeDirectory,

    #[error("{program} command failed: {program} {args:?}; stderr: {stderr}")]
    SystemCommand {
        program: String,
        args: Vec<String>,
        stderr: String,
        code: Option<i32>,
    },

    #[error("I/O error at {path}: {source}")]
    Io { path: PathBuf, source: io::Error },

    #[error("internal error: {message}")]
    Internal { message: String },
}

impl TideError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::ConfigParse { .. }
            | Self::InvalidTimezone { .. }
            | Self::ConfigExists { .. }
            | Self::InvalidReleaseTag { .. }
            | Self::InvalidServiceInterval { .. } => ExitCode::from(2),

            Self::NotGitRepository { .. }
            | Self::GitCommand { .. }
            | Self::SystemCommand { .. } => ExitCode::from(3),

            Self::NoReleaseAnchor { .. }
            | Self::TimestampAnomaly { .. }
            | Self::FileHistoryNotFound { .. } => ExitCode::from(4),

            Self::InvalidUtf8
            | Self::CacheFormat { .. }
            | Self::UnsupportedPlatform { .. }
            | Self::MissingHomeDirectory
            | Self::Io { .. }
            | Self::Internal { .. } => ExitCode::from(5),
        }
    }
}

pub fn io_err(path: impl Into<PathBuf>, source: io::Error) -> TideError {
    TideError::Io {
        path: path.into(),
        source,
    }
}
