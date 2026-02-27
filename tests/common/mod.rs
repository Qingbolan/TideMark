//! TideMark
//! ========
//!
//! File: tests/common/mod.rs
//! Description: Shared integration-test fixture utilities for temporary Git repositories.
//!
//! Responsibility:
//! - Provide deterministic repository setup, commit/tag helpers, and binary invocation wrappers.
//!
//! Architectural Position:
//! - Test support layer reused by all integration suites.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use assert_cmd::cargo;
use tempfile::TempDir;

pub struct RepoFixture {
    dir: TempDir,
}

#[allow(dead_code)]
impl RepoFixture {
    pub fn init() -> Self {
        let dir = tempfile::tempdir().expect("create temp dir");
        run_git(dir.path(), &["init", "-b", "main"], &[]);
        run_git(dir.path(), &["config", "user.name", "TideMark Test"], &[]);
        run_git(
            dir.path(),
            &["config", "user.email", "tidemark@test.local"],
            &[],
        );
        Self { dir }
    }

    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    pub fn write_file_and_commit(
        &self,
        rel_path: &str,
        content: &str,
        message: &str,
        iso_ts: &str,
    ) -> String {
        let path = self.root().join(rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(&path, content).expect("write file");
        run_git(self.root(), &["add", rel_path], &[]);
        run_git(
            self.root(),
            &["commit", "-m", message],
            &[("GIT_AUTHOR_DATE", iso_ts), ("GIT_COMMITTER_DATE", iso_ts)],
        );
        self.rev_parse("HEAD")
    }

    pub fn tag_annotated(&self, name: &str, message: &str, iso_ts: &str) {
        run_git(
            self.root(),
            &["tag", "-a", name, "-m", message],
            &[("GIT_COMMITTER_DATE", iso_ts)],
        );
    }

    pub fn tag_lightweight(&self, name: &str) {
        run_git(self.root(), &["tag", name], &[]);
    }

    pub fn rev_parse(&self, rev: &str) -> String {
        git_output(self.root(), &["rev-parse", rev])
            .trim()
            .to_string()
    }

    pub fn git_log_lines(&self, range: &str) -> Vec<String> {
        git_output(self.root(), &["log", "--format=%H%x09%ct", range])
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    pub fn run_tide(&self, args: &[&str]) -> Output {
        Command::new(cargo::cargo_bin!("tide"))
            .args(args)
            .current_dir(self.root())
            .output()
            .expect("run tidemark")
    }

    pub fn run_git_tide(&self, args: &[&str]) -> Output {
        Command::new(cargo::cargo_bin!("git-tide"))
            .args(args)
            .current_dir(self.root())
            .output()
            .expect("run git-tide")
    }

    pub fn run_git_subcommand_tide(&self, args: &[&str]) -> Output {
        let plugin_path = cargo::cargo_bin!("git-tide");
        let plugin_dir = plugin_path.parent().expect("plugin dir");
        let mut path_entries = vec![plugin_dir.as_os_str().to_os_string()];
        if let Some(existing) = env::var_os("PATH") {
            path_entries.extend(env::split_paths(&existing).map(PathBuf::into_os_string));
        }
        let joined_path = env::join_paths(path_entries).expect("join PATH");

        Command::new("git")
            .arg("tide")
            .args(args)
            .env("PATH", joined_path)
            .current_dir(self.root())
            .output()
            .expect("run git tide")
    }

    pub fn write_config(&self, raw: &str) {
        fs::write(self.root().join(".tidemark.toml"), raw).expect("write config");
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.root().join(".git").join("tidemark-cache")
    }
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
