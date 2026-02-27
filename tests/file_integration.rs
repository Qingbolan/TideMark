//! TideMark
//! ========
//!
//! File: tests/file_integration.rs
//! Description: Integration tests for file-path based coordinate resolution.
//!
//! Responsibility:
//! - Validate last-modifying-commit lookup and file-history failure behavior.
//!
//! Architectural Position:
//! - End-to-end verification of file resolver semantics.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

mod common;

use common::RepoFixture;

#[test]
fn file_uses_last_modifying_commit_coordinate() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("a.txt", "a1\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v3", "release 3", "2024-01-01T00:05:00+00:00");
    repo.write_file_and_commit("a.txt", "a2\n", "c2", "2024-01-01T01:00:00+00:00");
    repo.write_file_and_commit("b.txt", "b1\n", "c3", "2024-01-02T01:00:00+00:00");

    let output = repo.run_tide(&["file", "a.txt"]);
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1.0.1\n");
}

#[test]
fn file_without_history_returns_data_error() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("seed.txt", "x\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");

    let output = repo.run_tide(&["file", "missing.txt"]);
    assert_eq!(output.status.code(), Some(4));
}
