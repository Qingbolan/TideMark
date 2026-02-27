//! TideMark
//! ========
//!
//! File: tests/plugin_service_integration.rs
//! Description: Integration tests for Git plugin dispatch and service planning output.
//!
//! Responsibility:
//! - Ensure plugin parity with primary binary and deterministic service-plan rendering.
//!
//! Architectural Position:
//! - Cross-boundary verification for interface and operations integration.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

mod common;

use common::RepoFixture;

#[test]
fn git_tide_binary_matches_tide_output() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("a.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");
    repo.write_file_and_commit("a.txt", "b\n", "c2", "2024-01-01T01:00:00+00:00");

    let tide = repo.run_tide(&["mark", "--local-only"]);
    let git_tide = repo.run_git_tide(&["mark", "--local-only"]);

    assert!(
        tide.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&tide.stderr)
    );
    assert!(
        git_tide.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&git_tide.stderr)
    );
    assert_eq!(tide.stdout, git_tide.stdout);
}

#[test]
fn git_subcommand_dispatches_to_git_tide() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("a.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");
    repo.write_file_and_commit("a.txt", "b\n", "c2", "2024-01-01T01:00:00+00:00");

    let output = repo.run_git_subcommand_tide(&["mark", "--local-only"]);
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1.0.1\n");
}

#[test]
fn service_plan_is_deterministic_and_script_safe() {
    let repo = RepoFixture::init();

    repo.write_file_and_commit("seed.txt", "x\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");

    let output = repo.run_tide(&[
        "service",
        "plan",
        "--interval-minutes",
        "15",
        "--tag",
        "dev",
    ]);

    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("unit_name=tidemark-"));
    assert!(text.contains("---service---"));
    assert!(text.contains("---timer---"));
    assert!(text.contains("OnUnitActiveSec=15min"));
    assert!(text.contains("mark"));
    assert!(text.contains("--local-only"));
    assert!(text.contains("--explain"));
    assert!(text.contains("--tag"));
}
