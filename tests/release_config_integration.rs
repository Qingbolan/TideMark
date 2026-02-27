//! TideMark
//! ========
//!
//! File: tests/release_config_integration.rs
//! Description: Integration tests for release listing and config initialization workflows.
//!
//! Responsibility:
//! - Verify script-safe release rows and idempotent config init behavior.
//!
//! Architectural Position:
//! - End-to-end coverage for release query and config command boundaries.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

mod common;

use std::path::Path;

use common::RepoFixture;

#[test]
fn release_list_prints_script_safe_rows() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("a.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");
    repo.write_file_and_commit("a.txt", "b\n", "c2", "2024-01-02T00:00:00+00:00");
    repo.tag_annotated("v2", "release 2", "2024-01-02T00:00:00+00:00");

    let output = repo.run_tide(&["release", "list", "--local-only"]);
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("v1\t1\t"));
    assert!(text.contains("v2\t2\t"));
}

#[test]
fn config_init_creates_file_once() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("seed.txt", "x\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");

    let first = repo.run_tide(&["config", "init"]);
    assert!(
        first.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );

    let config_path = Path::new(repo.root()).join(".tidemark.toml");
    assert!(config_path.exists());

    let second = repo.run_tide(&["config", "init"]);
    assert_eq!(second.status.code(), Some(2));
}
