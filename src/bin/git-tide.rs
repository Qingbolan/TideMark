//! TideMark
//! ========
//!
//! File: src/bin/git-tide.rs
//! Description: Git plugin binary bootstrap for the `git tide` subcommand.
//!
//! Responsibility:
//! - Parse plugin arguments and dispatch execution to the shared application runner.
//!
//! Architectural Position:
//! - Executable entrypoint integrated with Git external subcommand discovery.
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
