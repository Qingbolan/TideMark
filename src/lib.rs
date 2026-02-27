//! TideMark
//! ========
//!
//! File: src/lib.rs
//! Description: Crate root module declarations for TideMark layered architecture.
//!
//! Responsibility:
//! - Expose application, core, infrastructure, interface, operations, and cross-cutting modules.
//!
//! Architectural Position:
//! - Top-level library assembly boundary for binaries and tests.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

pub mod app;
pub mod config;
pub mod core;
pub mod error;
pub mod infra;
pub mod interface;
pub mod ops;
