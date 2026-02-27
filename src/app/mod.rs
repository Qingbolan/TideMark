//! TideMark
//! ========
//!
//! File: src/app/mod.rs
//! Description: Application orchestration entry that wires CLI commands to core services and infrastructure adapters.
//!
//! Responsibility:
//! - Coordinate end-to-end command execution without duplicating domain algorithms.
//!
//! Architectural Position:
//! - Application layer facade between interface, core, infrastructure, and operations modules.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    env,
    io::{self, Write},
};

use crate::{
    config,
    core::{
        release,
        resolver::{
            file::{FileRequest, resolve_file},
            mark::{MarkRequest, resolve_mark},
        },
    },
    error::{TideResult, io_err},
    infra::{
        cache::CacheStore,
        git::{GitProvider, cli::GitCli},
    },
    interface::{
        cli::{Cli, Commands, ConfigSubcommand, ReleaseSubcommand, ServiceSubcommand},
        output,
    },
    ops::service::{self, ServiceInstallRequest, ServiceUninstallRequest},
};

pub fn run(cli: Cli) -> TideResult<()> {
    let cwd = env::current_dir().map_err(|err| io_err(".", err))?;
    let git = GitCli::discover(cwd.as_path())?;

    match cli.command {
        Commands::Config(config_cmd) => match config_cmd.command {
            ConfigSubcommand::Init => {
                let path = config::init_default(git.repo_root())?;
                write_stdout(format!("{}\n", path.display()).as_str())
            }
        },

        Commands::Mark(mark_args) => {
            let cfg = config::load_or_default(git.repo_root())?;
            let cache = CacheStore::new(git.git_dir()?.as_path(), cfg.cache.enabled);
            let result = resolve_mark(
                &git,
                &cfg,
                &cache,
                MarkRequest {
                    target_rev: None,
                    local_only: mark_args.local_only,
                    metadata_suffix: mark_args.metadata_suffix,
                },
            )?;
            write_stdout(output::format_mark(&result, mark_args.explain).as_str())
        }

        Commands::File(file_args) => {
            let cfg = config::load_or_default(git.repo_root())?;
            let cache = CacheStore::new(git.git_dir()?.as_path(), cfg.cache.enabled);
            let result = resolve_file(
                &git,
                &cfg,
                &cache,
                FileRequest {
                    path: file_args.path,
                    local_only: file_args.local_only,
                    metadata_suffix: file_args.metadata_suffix,
                },
            )?;
            write_stdout(output::format_file(&result).as_str())
        }

        Commands::Release(release_cmd) => match release_cmd.command {
            ReleaseSubcommand::List(args) => {
                let cfg = config::load_or_default(git.repo_root())?;
                let (releases, _remote_status) =
                    release::load_release_tags(&git, &cfg, args.local_only)?;
                write_stdout(output::format_release_list(&releases).as_str())
            }
        },

        Commands::Service(service_cmd) => match service_cmd.command {
            ServiceSubcommand::Install(args) => {
                let plan = service::install_user_service(&to_install_request(
                    git.repo_root(),
                    args.interval_minutes,
                    args.unit_name,
                    args.allow_remote,
                    args.compact,
                    args.metadata_suffix,
                ))?;
                write_stdout(
                    format!(
                        "unit_name={}\nservice_file={}\ntimer_file={}\n",
                        plan.unit_name,
                        plan.service_file.display(),
                        plan.timer_file.display()
                    )
                    .as_str(),
                )
            }
            ServiceSubcommand::Uninstall(args) => {
                let plan = service::uninstall_user_service(&ServiceUninstallRequest {
                    repo_root: git.repo_root().to_path_buf(),
                    unit_name: args.unit_name,
                })?;
                write_stdout(
                    format!(
                        "unit_name={}\nservice_file={}\ntimer_file={}\n",
                        plan.unit_name,
                        plan.service_file.display(),
                        plan.timer_file.display()
                    )
                    .as_str(),
                )
            }
            ServiceSubcommand::Plan(args) => {
                let plan = service::plan_service(&to_install_request(
                    git.repo_root(),
                    args.interval_minutes,
                    args.unit_name,
                    args.allow_remote,
                    args.compact,
                    args.metadata_suffix,
                ))?;
                write_stdout(
                    format!(
                        "unit_name={}\nservice_file={}\ntimer_file={}\n---service---\n{}---timer---\n{}",
                        plan.unit_name,
                        plan.service_file.display(),
                        plan.timer_file.display(),
                        plan.service_content,
                        plan.timer_content,
                    )
                    .as_str(),
                )
            }
        },
    }
}

fn to_install_request(
    repo_root: &std::path::Path,
    interval_minutes: u32,
    unit_name: Option<String>,
    allow_remote: bool,
    compact: bool,
    metadata_suffix: Option<String>,
) -> ServiceInstallRequest {
    ServiceInstallRequest {
        repo_root: repo_root.to_path_buf(),
        interval_minutes,
        unit_name,
        local_only: !allow_remote,
        explain: !compact,
        metadata_suffix,
    }
}

fn write_stdout(text: &str) -> TideResult<()> {
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(text.as_bytes())
        .map_err(|err| io_err("stdout", err))
}
