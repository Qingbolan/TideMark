//! TideMark
//! ========
//!
//! File: src/core/mod.rs
//! Description: Core domain module index for deterministic version-coordinate semantics.
//!
//! Responsibility:
//! - Expose domain model, time policy, release-anchor selection, and resolver algorithms.
//!
//! Architectural Position:
//! - Pure domain layer isolated from CLI rendering and operational side effects.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

pub mod model;
pub mod release;
pub mod resolver;
pub mod time;
