//! TideMark
//! ========
//!
//! File: tests/remote_refresh_integration.rs
//! Description: Integration tests for remote tag refresh, cache bypass, and collision precedence rules.
//!
//! Responsibility:
//! - Validate non-local mode freshness and local-only isolation after remote tag updates.
//!
//! Architectural Position:
//! - Regression coverage for remote synchronization semantics.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

use assert_cmd::cargo;

#[test]
fn remote_mode_refreshes_latest_tags_without_stale_cache() {
    let sandbox = tempfile::tempdir().expect("create sandbox");
    let remote = sandbox.path().join("remote.git");
    let upstream = sandbox.path().join("upstream");
    let local = sandbox.path().join("local");

    run_git(sandbox.path(), &["init", "--bare", "remote.git"], &[]);
    init_repo(&upstream);

    write_and_commit(
        &upstream,
        "app.txt",
        "hello\n",
        "c1",
        "2024-01-01T00:00:00+00:00",
    );
    run_git(
        &upstream,
        &["tag", "-a", "v1", "-m", "release 1"],
        &[("GIT_COMMITTER_DATE", "2024-01-01T00:10:00+00:00")],
    );
    run_git(
        &upstream,
        &["remote", "add", "origin", path_text(&remote)],
        &[],
    );
    run_git(&upstream, &["push", "origin", "main", "--tags"], &[]);
    init_local_checkout_from_remote(&remote, &local);

    let first = run_tide(&local, &["mark"]);
    assert_success(&first);
    assert_eq!(stdout_text(&first), "1.0.0\n");

    run_git(
        &upstream,
        &["tag", "-a", "v2", "-m", "release 2"],
        &[("GIT_COMMITTER_DATE", "2024-01-01T00:20:00+00:00")],
    );
    run_git(&upstream, &["push", "origin", "v2"], &[]);

    let second = run_tide(&local, &["mark"]);
    assert_success(&second);
    assert_eq!(stdout_text(&second), "2.0.0\n");

    let local_only = run_tide(&local, &["mark", "--local-only"]);
    assert_success(&local_only);
    assert_eq!(stdout_text(&local_only), "1.0.0\n");
}

#[test]
fn remote_same_name_tag_overrides_local_definition() {
    let sandbox = tempfile::tempdir().expect("create sandbox");
    let remote = sandbox.path().join("remote.git");
    let upstream = sandbox.path().join("upstream");
    let local = sandbox.path().join("local");

    run_git(sandbox.path(), &["init", "--bare", "remote.git"], &[]);
    init_repo(&upstream);

    write_and_commit(
        &upstream,
        "app.txt",
        "a\n",
        "c1",
        "2024-01-01T00:00:00+00:00",
    );
    run_git(
        &upstream,
        &["tag", "-a", "v1", "-m", "release 1"],
        &[("GIT_COMMITTER_DATE", "2024-01-01T00:10:00+00:00")],
    );
    write_and_commit(
        &upstream,
        "app.txt",
        "b\n",
        "c2",
        "2024-01-02T00:00:00+00:00",
    );
    write_and_commit(
        &upstream,
        "app.txt",
        "c\n",
        "c3",
        "2024-01-03T00:00:00+00:00",
    );

    run_git(
        &upstream,
        &["remote", "add", "origin", path_text(&remote)],
        &[],
    );
    run_git(&upstream, &["push", "origin", "main", "--tags"], &[]);
    init_local_checkout_from_remote(&remote, &local);

    let baseline_local_only = run_tide(&local, &["mark", "--local-only"]);
    assert_success(&baseline_local_only);
    assert_eq!(stdout_text(&baseline_local_only), "1.2.1\n");

    let c2 = git_output(&upstream, &["rev-parse", "HEAD~1"]);
    run_git(
        &upstream,
        &["tag", "-fa", "v1", "-m", "release 1 moved", c2.trim()],
        &[("GIT_COMMITTER_DATE", "2024-01-04T00:00:00+00:00")],
    );
    run_git(
        &upstream,
        &["push", "origin", "+refs/tags/v1:refs/tags/v1"],
        &[],
    );

    let remote_mode = run_tide(&local, &["mark"]);
    assert_success(&remote_mode);
    assert_eq!(stdout_text(&remote_mode), "1.1.1\n");

    let local_only_after = run_tide(&local, &["mark", "--local-only"]);
    assert_success(&local_only_after);
    assert_eq!(stdout_text(&local_only_after), "1.2.1\n");
}

fn init_repo(path: &Path) {
    fs::create_dir_all(path).expect("create repo dir");
    run_git(path, &["init", "-b", "main"], &[]);
    run_git(path, &["config", "user.name", "TideMark Test"], &[]);
    run_git(path, &["config", "user.email", "tidemark@test.local"], &[]);
}

fn write_and_commit(repo: &Path, rel_path: &str, content: &str, message: &str, iso_ts: &str) {
    let file_path = repo.join(rel_path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(&file_path, content).expect("write file");
    run_git(repo, &["add", rel_path], &[]);
    run_git(
        repo,
        &["commit", "-m", message],
        &[("GIT_AUTHOR_DATE", iso_ts), ("GIT_COMMITTER_DATE", iso_ts)],
    );
}

fn init_local_checkout_from_remote(remote: &Path, local: &Path) {
    fs::create_dir_all(local).expect("create local dir");
    run_git(local, &["init", "-b", "main"], &[]);
    run_git(local, &["remote", "add", "origin", path_text(remote)], &[]);
    run_git(local, &["fetch", "--tags", "origin", "main"], &[]);
    run_git(local, &["checkout", "-B", "main", "FETCH_HEAD"], &[]);
}

fn run_tide(repo: &Path, args: &[&str]) -> Output {
    Command::new(cargo::cargo_bin!("tide"))
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run tide")
}

fn run_git(dir: &Path, args: &[&str], envs: &[(&str, &str)]) {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(dir).args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }

    let output = cmd.output().expect("run git");
    if !output.status.success() {
        panic!(
            "git command failed: git {:?}\nstdout: {}\nstderr: {}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn git_output(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("run git output");

    if !output.status.success() {
        panic!(
            "git command failed: git {:?}\nstdout: {}\nstderr: {}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).expect("utf8")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout utf8")
}

fn path_text(path: &Path) -> &str {
    path.to_str().expect("utf8 path")
}
