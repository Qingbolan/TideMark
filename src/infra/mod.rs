//! TideMark
//! ========
//!
//! File: src/infra/mod.rs
//! Description: Infrastructure module index.
//!
//! Responsibility:
//! - Expose cache and Git backends that interact with filesystem and external commands.
//!
//! Architectural Position:
//! - I/O boundary layer for persistence and repository inspection.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

pub mod cache;
pub mod git;
