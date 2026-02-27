//! TideMark
//! ========
//!
//! File: src/infra/git/cli.rs
//! Description: Git CLI backend implementing the `GitProvider` trait through shell command execution.
//!
//! Responsibility:
//! - Resolve commits, tags, ancestry paths, and remote-refresh behavior using deterministic Git invocations.
//!
//! Architectural Position:
//! - Concrete infrastructure adapter for repository state access and refresh.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::{
    core::model::{CommitInfo, TagRef, TagSource},
    error::{TideError, TideResult, io_err},
    infra::git::GitProvider,
};

#[derive(Debug, Clone)]
pub struct GitCli {
    repo_root: PathBuf,
}

impl GitCli {
    pub fn discover(start_dir: &Path) -> TideResult<Self> {
        let output = run_git_at(start_dir, &["rev-parse", "--show-toplevel"])?;
        if !output.status.success() {
            return Err(TideError::NotGitRepository {
                path: start_dir.to_path_buf(),
            });
        }
        let root = stdout_trimmed(&output)?;
        Ok(Self {
            repo_root: PathBuf::from(root),
        })
    }

    fn run_git(&self, args: &[&str]) -> TideResult<Output> {
        run_git_at(&self.repo_root, args)
    }

    fn run_git_checked(&self, args: &[&str]) -> TideResult<String> {
        let output = self.run_git(args)?;
        if output.status.success() {
            return stdout_trimmed(&output);
        }

        Err(TideError::GitCommand {
            args: args.iter().map(|v| v.to_string()).collect(),
            stderr: stderr_trimmed(&output),
            code: output.status.code(),
        })
    }

    fn parse_commit_line(&self, line: &str) -> TideResult<CommitInfo> {
        let mut parts = line.split('\t');
        let id = parts.next().unwrap_or_default().trim();
        let timestamp = parts.next().unwrap_or_default().trim();
        if id.is_empty() || timestamp.is_empty() {
            return Err(TideError::Internal {
                message: format!("unexpected commit line format: {line}"),
            });
        }
        let timestamp = timestamp.parse::<i64>().map_err(|_| TideError::Internal {
            message: format!("invalid commit timestamp: {timestamp}"),
        })?;
        Ok(CommitInfo {
            id: id.to_string(),
            timestamp,
        })
    }
}

impl GitProvider for GitCli {
    fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    fn git_dir(&self) -> TideResult<PathBuf> {
        let out = self.run_git_checked(&["rev-parse", "--git-dir"])?;
        let path = PathBuf::from(out);
        if path.is_absolute() {
            Ok(path)
        } else {
            Ok(self.repo_root.join(path))
        }
    }

    fn head_commit(&self) -> TideResult<CommitInfo> {
        self.resolve_commit("HEAD")
    }

    fn resolve_commit(&self, rev: &str) -> TideResult<CommitInfo> {
        let out = self.run_git_checked(&["show", "-s", "--format=%H%x09%ct", rev])?;
        self.parse_commit_line(out.as_str())
    }

    fn commit_exists(&self, rev: &str) -> TideResult<bool> {
        let output = self.run_git(&["cat-file", "-e", &format!("{rev}^{{commit}}")])?;
        if output.status.success() {
            return Ok(true);
        }
        if output.status.code() == Some(128) {
            return Ok(false);
        }
        Err(TideError::GitCommand {
            args: vec![
                "cat-file".to_string(),
                "-e".to_string(),
                format!("{rev}^{{commit}}"),
            ],
            stderr: stderr_trimmed(&output),
            code: output.status.code(),
        })
    }

    fn list_local_tags(&self, prefix: &str) -> TideResult<Vec<TagRef>> {
        let out = self.run_git_checked(&[
            "for-each-ref",
            "--format=%(refname:short)%09%(objecttype)%09%(*objectname)%09%(objectname)",
            "refs/tags",
        ])?;

        let mut tags = Vec::new();
        for line in out.lines().filter(|line| !line.trim().is_empty()) {
            let mut fields = line.split('\t');
            let name = fields.next().unwrap_or_default().to_string();
            let object_type = fields.next().unwrap_or_default();
            let peeled = fields.next().unwrap_or_default();
            let object = fields.next().unwrap_or_default();

            if !name.starts_with(prefix) {
                continue;
            }

            let (commit_id, is_annotated) = match object_type {
                "tag" => (peeled.to_string(), true),
                "commit" => (object.to_string(), false),
                _ => continue,
            };

            if commit_id.is_empty() {
                continue;
            }

            tags.push(TagRef {
                name,
                commit_id,
                is_annotated,
                source: TagSource::Local,
            });
        }

        Ok(tags)
    }

    fn list_remote_tags(&self, remote: &str, prefix: &str) -> TideResult<Vec<TagRef>> {
        let refspec = format!("+refs/tags/{prefix}*:refs/tidemark/remote-tags/{prefix}*");
        let output =
            self.run_git(&["fetch", "--quiet", "--prune", "--no-tags", remote, &refspec])?;
        if !output.status.success() {
            return Err(TideError::GitCommand {
                args: vec![
                    "fetch".to_string(),
                    "--quiet".to_string(),
                    "--prune".to_string(),
                    "--no-tags".to_string(),
                    remote.to_string(),
                    refspec,
                ],
                stderr: stderr_trimmed(&output),
                code: output.status.code(),
            });
        }

        let out = self.run_git_checked(&[
            "for-each-ref",
            "--format=%(refname)%09%(objecttype)%09%(*objectname)%09%(objectname)",
            "refs/tidemark/remote-tags",
        ])?;

        let mut tags = Vec::new();
        for line in out.lines().filter(|line| !line.trim().is_empty()) {
            let mut fields = line.split('\t');
            let ref_name = fields.next().unwrap_or_default();
            let object_type = fields.next().unwrap_or_default();
            let peeled = fields.next().unwrap_or_default();
            let object = fields.next().unwrap_or_default();

            let Some(name) = ref_name.strip_prefix("refs/tidemark/remote-tags/") else {
                continue;
            };
            if !name.starts_with(prefix) {
                continue;
            }

            let (commit_id, is_annotated) = match object_type {
                "tag" => (peeled.to_string(), true),
                "commit" => (object.to_string(), false),
                _ => continue,
            };

            if commit_id.is_empty() {
                continue;
            }

            tags.push(TagRef {
                name: name.to_string(),
                commit_id,
                is_annotated,
                source: TagSource::Remote,
            });
        }

        Ok(tags)
    }

    fn is_ancestor(&self, ancestor: &str, descendant: &str) -> TideResult<bool> {
        let output = self.run_git(&["merge-base", "--is-ancestor", ancestor, descendant])?;
        match output.status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => Err(TideError::GitCommand {
                args: vec![
                    "merge-base".to_string(),
                    "--is-ancestor".to_string(),
                    ancestor.to_string(),
                    descendant.to_string(),
                ],
                stderr: stderr_trimmed(&output),
                code: output.status.code(),
            }),
        }
    }

    fn commit_distance(&self, ancestor: &str, descendant: &str) -> TideResult<u32> {
        let range = format!("{ancestor}..{descendant}");
        let out = self.run_git_checked(&["rev-list", "--count", &range])?;
        out.parse::<u32>().map_err(|_| TideError::Internal {
            message: format!("invalid commit count output: {out}"),
        })
    }

    fn ancestry_path_commits(
        &self,
        ancestor: &str,
        descendant: &str,
    ) -> TideResult<Vec<CommitInfo>> {
        if ancestor == descendant {
            return Ok(Vec::new());
        }
        let range = format!("{ancestor}..{descendant}");
        let out = self.run_git_checked(&[
            "log",
            "--ancestry-path",
            "--reverse",
            "--format=%H%x09%ct",
            &range,
        ])?;
        let mut commits = Vec::new();
        for line in out.lines().filter(|line| !line.trim().is_empty()) {
            commits.push(self.parse_commit_line(line)?);
        }
        Ok(commits)
    }

    fn last_modifying_commit(&self, path: &Path, follow_renames: bool) -> TideResult<CommitInfo> {
        let normalized_path = if path.is_absolute() {
            path.strip_prefix(&self.repo_root)
                .unwrap_or(path)
                .to_path_buf()
        } else {
            path.to_path_buf()
        };

        let path_arg = normalized_path.to_string_lossy().to_string();
        let mut args = vec!["log", "-n", "1", "--format=%H%x09%ct"];
        if follow_renames {
            args.push("--follow");
        }
        args.push("--");
        args.push(path_arg.as_str());

        let output = self.run_git(&args)?;
        if !output.status.success() {
            return Err(TideError::GitCommand {
                args: args.iter().map(|s| s.to_string()).collect(),
                stderr: stderr_trimmed(&output),
                code: output.status.code(),
            });
        }

        let out = stdout_trimmed(&output)?;
        if out.trim().is_empty() {
            return Err(TideError::FileHistoryNotFound {
                path: normalized_path,
            });
        }
        self.parse_commit_line(out.as_str())
    }

    fn root_commit(&self) -> TideResult<CommitInfo> {
        let out = self.run_git_checked(&["rev-list", "--max-parents=0", "HEAD"])?;
        let first_root = out
            .lines()
            .next()
            .ok_or_else(|| TideError::Internal {
                message: "no root commit found".to_string(),
            })?
            .trim();
        self.resolve_commit(first_root)
    }

    fn current_branch(&self) -> TideResult<Option<String>> {
        let output = self.run_git(&["symbolic-ref", "--quiet", "--short", "HEAD"])?;
        match output.status.code() {
            Some(0) => Ok(Some(stdout_trimmed(&output)?)),
            Some(1) => Ok(None),
            _ => Err(TideError::GitCommand {
                args: vec![
                    "symbolic-ref".to_string(),
                    "--quiet".to_string(),
                    "--short".to_string(),
                    "HEAD".to_string(),
                ],
                stderr: stderr_trimmed(&output),
                code: output.status.code(),
            }),
        }
    }
}

fn run_git_at(dir: &Path, args: &[&str]) -> TideResult<Output> {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|err| io_err(dir, err))
}

fn stdout_trimmed(output: &Output) -> TideResult<String> {
    let s = String::from_utf8(output.stdout.clone()).map_err(|_| TideError::InvalidUtf8)?;
    Ok(s.trim().to_string())
}

fn stderr_trimmed(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}
