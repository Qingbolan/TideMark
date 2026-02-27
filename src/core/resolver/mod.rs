//! TideMark
//! ========
//!
//! File: src/core/resolver/mod.rs
//! Description: Resolver module index for commit and file version-coordinate workflows.
//!
//! Responsibility:
//! - Expose mark and file resolution paths that produce deterministic version coordinates.
//!
//! Architectural Position:
//! - Core orchestration sublayer dedicated to coordinate computation pipelines.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

pub mod file;
pub mod mark;
