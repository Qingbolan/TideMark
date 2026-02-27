//! TideMark
//! ========
//!
//! File: src/bin/tide.rs
//! Description: Primary TideMark CLI binary bootstrap.
//!
//! Responsibility:
//! - Parse command-line input and dispatch execution to the application runner.
//!
//! Architectural Position:
//! - Executable entrypoint for direct TideMark command usage.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::process::ExitCode;

use clap::Parser;
use tidemark::{app, interface::cli::Cli};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match app::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            err.exit_code()
        }
    }
}
