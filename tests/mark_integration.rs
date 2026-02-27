//! TideMark
//! ========
//!
//! File: tests/mark_integration.rs
//! Description: Integration tests for mark command coordinate generation and explain output.
//!
//! Responsibility:
//! - Validate baseline coordinate behavior, explain fields, and annotated-tag policy controls.
//!
//! Architectural Position:
//! - End-to-end verification of commit mark semantics.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

mod common;

use common::RepoFixture;

#[test]
fn mark_resolves_expected_coordinate() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("app.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:10:00+00:00");
    repo.write_file_and_commit("app.txt", "b\n", "c2", "2024-01-01T01:00:00+00:00");
    repo.write_file_and_commit("app.txt", "c\n", "c3", "2024-01-02T01:00:00+00:00");

    let output = repo.run_tide(&["mark"]);
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1.1.1\n");
}

#[test]
fn mark_explain_is_key_value() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("a.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v2", "release 2", "2024-01-01T00:00:00+00:00");
    repo.write_file_and_commit("a.txt", "b\n", "c2", "2024-01-01T02:00:00+00:00");

    let output = repo.run_tide(&["mark", "--explain"]);
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("version=1.0.1"));
    assert!(text.contains("anchor_tag=v2"));
    assert!(text.contains("day_delta=0"));
    assert!(text.contains("commit_index=1"));
}

#[test]
fn lightweight_tags_rejected_by_default_but_configurable() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("a.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_lightweight("v1");
    repo.write_file_and_commit("a.txt", "b\n", "c2", "2024-01-01T01:00:00+00:00");

    let default_output = repo.run_tide(&["mark"]);
    assert!(
        default_output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&default_output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&default_output.stdout),
        "0.0.1\n"
    );

    repo.write_config("[release]\nrequire_annotated_tags = false\n\n[time]\ntimezone = \"UTC\"\n");

    let configured_output = repo.run_tide(&["mark"]);
    assert!(configured_output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&configured_output.stdout),
        "1.0.1\n"
    );
}
