//! TideMark
//! ========
//!
//! File: tests/determinism_regression.rs
//! Description: Determinism regression tests for stable repeated mark outputs.
//!
//! Responsibility:
//! - Verify byte-equal results and cache creation under repeated local-only resolution.
//!
//! Architectural Position:
//! - Integration verification for deterministic resolver guarantees.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

mod common;

use common::RepoFixture;

#[test]
fn repeated_runs_are_byte_equal() {
    let repo = RepoFixture::init();

    let anchor = repo.write_file_and_commit("x.txt", "a\n", "c1", "2024-01-01T00:00:00+00:00");
    repo.tag_annotated("v1", "release 1", "2024-01-01T00:00:00+00:00");
    repo.write_file_and_commit("x.txt", "b\n", "c2", "2024-01-01T10:00:00+00:00");
    repo.write_file_and_commit("x.txt", "c\n", "c3", "2024-01-01T10:00:00+00:00");

    let first = repo.run_tide(&["mark", "--local-only"]);
    let second = repo.run_tide(&["mark", "--local-only"]);

    assert!(
        first.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(first.stdout, second.stdout);

    let head = repo.rev_parse("HEAD");
    let mut same_day_hashes: Vec<String> = repo
        .git_log_lines(&format!("{anchor}..{head}"))
        .iter()
        .filter_map(|line| line.split('\t').next().map(|s| s.to_string()))
        .collect();
    same_day_hashes.sort();

    let expected_z = same_day_hashes
        .iter()
        .position(|hash| hash == &head)
        .expect("head should exist")
        + 1;

    let expected = format!("1.0.{}\n", expected_z);
    assert_eq!(String::from_utf8_lossy(&first.stdout), expected);

    assert!(
        repo.cache_dir().exists(),
        "cache directory should be created"
    );
}
