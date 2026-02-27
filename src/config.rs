//! TideMark
//! ========
//!
//! File: src/config.rs
//! Description: Configuration schema, defaults, parsing, and initialization utilities.
//!
//! Responsibility:
//! - Provide deterministic runtime configuration with validated defaults and explicit file loading behavior.
//!
//! Architectural Position:
//! - Cross-cutting configuration boundary consumed by application, core, and operations flows.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::{TideError, TideResult, io_err};

pub const CONFIG_FILE_NAME: &str = ".tidemark.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TideConfig {
    #[serde(default)]
    pub release: ReleaseConfig,
    #[serde(default)]
    pub time: TimeConfig,
    #[serde(default)]
    pub remote: RemoteConfig,
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub output: OutputConfig,
}

impl Default for TideConfig {
    fn default() -> Self {
        Self {
            release: ReleaseConfig::default(),
            time: TimeConfig::default(),
            remote: RemoteConfig::default(),
            cache: CacheConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseConfig {
    #[serde(default = "default_tag_prefix")]
    pub tag_prefix: String,
    #[serde(default = "default_true")]
    pub require_annotated_tags: bool,
}

impl Default for ReleaseConfig {
    fn default() -> Self {
        Self {
            tag_prefix: default_tag_prefix(),
            require_annotated_tags: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConfig {
    #[serde(default = "default_timezone")]
    pub timezone: String,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            timezone: default_timezone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    #[serde(default)]
    pub strategy: RemoteStrategy,
    #[serde(default = "default_remote_name")]
    pub name: String,
    #[serde(default = "default_true")]
    pub fallback_to_local: bool,
}

impl Default for RemoteConfig {
    fn default() -> Self {
        Self {
            strategy: RemoteStrategy::LsRemote,
            name: default_remote_name(),
            fallback_to_local: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteStrategy {
    #[default]
    LsRemote,
    LocalOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default)]
    pub metadata_suffix: Option<String>,
    #[serde(default = "default_true")]
    pub follow_renames: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            metadata_suffix: None,
            follow_renames: true,
        }
    }
}

fn default_tag_prefix() -> String {
    "v".to_string()
}

fn default_timezone() -> String {
    "UTC".to_string()
}

fn default_remote_name() -> String {
    "origin".to_string()
}

const fn default_true() -> bool {
    true
}

pub fn load_or_default(repo_root: &Path) -> TideResult<TideConfig> {
    let path = repo_root.join(CONFIG_FILE_NAME);
    if !path.exists() {
        return Ok(TideConfig::default());
    }

    let raw = fs::read_to_string(&path).map_err(|err| io_err(&path, err))?;
    toml::from_str::<TideConfig>(&raw).map_err(|err| TideError::ConfigParse {
        path,
        message: err.to_string(),
    })
}

pub fn init_default(repo_root: &Path) -> TideResult<PathBuf> {
    let path = repo_root.join(CONFIG_FILE_NAME);
    if path.exists() {
        return Err(TideError::ConfigExists { path });
    }

    fs::write(&path, default_config_toml()).map_err(|err| io_err(&path, err))?;
    Ok(path)
}

pub fn default_config_toml() -> &'static str {
    "# TideMark configuration\n\n[release]\ntag_prefix = \"v\"\nrequire_annotated_tags = true\n\n[time]\ntimezone = \"UTC\"\n\n[remote]\nstrategy = \"ls-remote\"\nname = \"origin\"\nfallback_to_local = true\n\n[cache]\nenabled = true\n\n[output]\n# Optional suffix appended as x.y.z.<suffix>; does not change coordinates\nmetadata_suffix = \"\"\nfollow_renames = true\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_remote_strategy() {
        let cfg: TideConfig = toml::from_str("[remote]\n").expect("parse config");
        assert_eq!(cfg.remote.strategy, RemoteStrategy::LsRemote);
    }
}
