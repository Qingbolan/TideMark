//! TideMark
//! ========
//!
//! File: src/infra/cache.rs
//! Description: Filesystem-backed cache store under `.git/tidemark-cache`.
//!
//! Responsibility:
//! - Provide namespaced cache keying plus atomic write semantics for deterministic command acceleration.
//!
//! Architectural Position:
//! - Infrastructure persistence adapter used by resolver workflows.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

use crate::error::{TideError, TideResult, io_err};

#[derive(Debug, Clone)]
pub struct CacheStore {
    root: PathBuf,
    enabled: bool,
}

impl CacheStore {
    pub fn new(git_dir: &Path, enabled: bool) -> Self {
        Self {
            root: git_dir.join("tidemark-cache"),
            enabled,
        }
    }

    pub fn key_from_serializable<T: Serialize>(namespace: &str, value: &T) -> TideResult<String> {
        let payload = serde_json::to_vec(value).map_err(|err| TideError::CacheFormat {
            message: err.to_string(),
        })?;

        let mut hasher = Sha256::new();
        hasher.update(namespace.as_bytes());
        hasher.update([0u8]);
        hasher.update(payload);
        Ok(hex::encode(hasher.finalize()))
    }

    pub fn get<T: DeserializeOwned>(&self, namespace: &str, key: &str) -> TideResult<Option<T>> {
        if !self.enabled {
            return Ok(None);
        }
        let path = self.path_for(namespace, key);
        if !path.exists() {
            return Ok(None);
        }

        let raw = fs::read_to_string(&path).map_err(|err| io_err(&path, err))?;
        let value = serde_json::from_str::<T>(&raw).map_err(|err| TideError::CacheFormat {
            message: format!("{}: {}", path.display(), err),
        })?;
        Ok(Some(value))
    }

    pub fn put<T: Serialize>(&self, namespace: &str, key: &str, value: &T) -> TideResult<()> {
        if !self.enabled {
            return Ok(());
        }

        let dir = self.root.join(namespace);
        fs::create_dir_all(&dir).map_err(|err| io_err(&dir, err))?;

        let path = self.path_for(namespace, key);
        let tmp = path.with_extension("tmp");
        let payload = serde_json::to_vec(value).map_err(|err| TideError::CacheFormat {
            message: err.to_string(),
        })?;

        fs::write(&tmp, payload).map_err(|err| io_err(&tmp, err))?;
        fs::rename(&tmp, &path).map_err(|err| io_err(&path, err))?;
        Ok(())
    }

    fn path_for(&self, namespace: &str, key: &str) -> PathBuf {
        self.root.join(namespace).join(format!("{key}.json"))
    }
}
