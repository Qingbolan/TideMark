//! TideMark
//! ========
//!
//! File: src/interface/cli.rs
//! Description: Command-line schema definitions using Clap.
//!
//! Responsibility:
//! - Declare stable command and flag contracts for mark, file, release, config, and service operations.
//!
//! Architectural Position:
//! - Interface input boundary consumed by binary entrypoints.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tide")]
#[command(version)]
#[command(about = "Git-native deterministic version coordinates")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Resolve version coordinate for HEAD
    Mark(MarkArgs),
    /// Resolve version coordinate for the last commit that modified <path>
    File(FileArgs),
    /// Release-anchor queries
    Release(ReleaseCommand),
    /// Configuration commands
    Config(ConfigCommand),
    /// Systemd user service management
    Service(ServiceCommand),
}

#[derive(Debug, clap::Args)]
pub struct MarkArgs {
    /// Print deterministic explain output (key=value lines)
    #[arg(long)]
    pub explain: bool,
    /// Disable remote tag query and use only local tags
    #[arg(long)]
    pub local_only: bool,
    /// Optional metadata suffix appended as x.y.z.<tag>
    #[arg(long = "tag")]
    pub metadata_suffix: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct FileArgs {
    pub path: PathBuf,
    /// Disable remote tag query and use only local tags
    #[arg(long)]
    pub local_only: bool,
    /// Optional metadata suffix appended as x.y.z.<tag>
    #[arg(long = "tag")]
    pub metadata_suffix: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct ReleaseCommand {
    #[command(subcommand)]
    pub command: ReleaseSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ReleaseSubcommand {
    /// List release tags recognized by TideMark
    List(ReleaseListArgs),
}

#[derive(Debug, clap::Args)]
pub struct ReleaseListArgs {
    /// Disable remote tag query and use only local tags
    #[arg(long)]
    pub local_only: bool,
}

#[derive(Debug, clap::Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {
    /// Create .tidemark.toml if absent
    Init,
}

#[derive(Debug, clap::Args)]
pub struct ServiceCommand {
    #[command(subcommand)]
    pub command: ServiceSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ServiceSubcommand {
    /// Install and start a user-level systemd timer for TideMark
    Install(ServiceInstallArgs),
    /// Uninstall and stop the user-level systemd timer
    Uninstall(ServiceUninstallArgs),
    /// Print deterministic unit and timer contents without installing
    Plan(ServicePlanArgs),
}

#[derive(Debug, clap::Args)]
pub struct ServiceInstallArgs {
    /// Timer interval in minutes; must be >= 1
    #[arg(long, default_value_t = 60)]
    pub interval_minutes: u32,
    /// Optional explicit systemd unit name (without .service/.timer)
    #[arg(long)]
    pub unit_name: Option<String>,
    /// Allow remote tag lookup during scheduled mark calculation
    #[arg(long)]
    pub allow_remote: bool,
    /// Output compact coordinate only (without --explain)
    #[arg(long)]
    pub compact: bool,
    /// Optional metadata suffix passed as --tag
    #[arg(long = "tag")]
    pub metadata_suffix: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct ServiceUninstallArgs {
    /// Optional explicit systemd unit name (without .service/.timer)
    #[arg(long)]
    pub unit_name: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct ServicePlanArgs {
    /// Timer interval in minutes; must be >= 1
    #[arg(long, default_value_t = 60)]
    pub interval_minutes: u32,
    /// Optional explicit systemd unit name (without .service/.timer)
    #[arg(long)]
    pub unit_name: Option<String>,
    /// Allow remote tag lookup during scheduled mark calculation
    #[arg(long)]
    pub allow_remote: bool,
    /// Output compact coordinate only (without --explain)
    #[arg(long)]
    pub compact: bool,
    /// Optional metadata suffix passed as --tag
    #[arg(long = "tag")]
    pub metadata_suffix: Option<String>,
}
