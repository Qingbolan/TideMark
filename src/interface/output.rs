//! TideMark
//! ========
//!
//! File: src/interface/output.rs
//! Description: Output formatting utilities for mark, file, and release query results.
//!
//! Responsibility:
//! - Render deterministic script-safe text surfaces from core model data.
//!
//! Architectural Position:
//! - Interface output boundary for human and automation consumption.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use crate::core::model::{FileResult, MarkResult, ReleaseTag};

pub fn format_mark(mark: &MarkResult, explain: bool) -> String {
    if !explain {
        return format!("{}\n", mark.coordinate);
    }

    let branch = mark
        .explain
        .branch
        .clone()
        .unwrap_or_else(|| "detached".to_string());

    [
        format!("version={}", mark.explain.version),
        format!("anchor_tag={}", mark.explain.anchor_tag),
        format!("anchor_commit={}", mark.explain.anchor_commit.id),
        format!("anchor_timestamp={}", mark.explain.anchor_commit.timestamp),
        format!("target_commit={}", mark.explain.target_commit.id),
        format!("target_timestamp={}", mark.explain.target_commit.timestamp),
        format!("day_delta={}", mark.explain.day_delta),
        format!("commit_index={}", mark.explain.commit_index),
        format!("timezone={}", mark.explain.timezone),
        format!("branch={branch}"),
        format!("remote_status={}", mark.explain.remote_status),
    ]
    .join("\n")
        + "\n"
}

pub fn format_file(file: &FileResult) -> String {
    format!("{}\n", file.mark.coordinate)
}

pub fn format_release_list(releases: &[ReleaseTag]) -> String {
    let mut lines = Vec::with_capacity(releases.len());
    for release in releases {
        lines.push(format!(
            "{}\t{}\t{}\t{}\t{}",
            release.tag.name,
            release.anchor_value,
            release.tag.commit_id,
            if release.tag.is_annotated {
                "annotated"
            } else {
                "lightweight"
            },
            release.tag.source
        ));
    }

    if lines.is_empty() {
        String::new()
    } else {
        lines.join("\n") + "\n"
    }
}
