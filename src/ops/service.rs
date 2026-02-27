//! TideMark
//! ========
//!
//! File: src/ops/service.rs
//! Description: Systemd user-service planning, installation, and uninstallation support.
//!
//! Responsibility:
//! - Generate deterministic unit files and manage service lifecycle via systemctl.
//!
//! Architectural Position:
//! - Operational adapter for periodic TideMark execution in Linux user sessions.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::error::{TideError, TideResult, io_err};

#[derive(Debug, Clone)]
pub struct ServicePlan {
    pub unit_name: String,
    pub service_file: PathBuf,
    pub timer_file: PathBuf,
    pub service_content: String,
    pub timer_content: String,
}

#[derive(Debug, Clone)]
pub struct ServiceInstallRequest {
    pub repo_root: PathBuf,
    pub interval_minutes: u32,
    pub unit_name: Option<String>,
    pub local_only: bool,
    pub explain: bool,
    pub metadata_suffix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ServiceUninstallRequest {
    pub repo_root: PathBuf,
    pub unit_name: Option<String>,
}

pub fn plan_service(request: &ServiceInstallRequest) -> TideResult<ServicePlan> {
    validate_interval(request.interval_minutes)?;

    let unit_name = request
        .unit_name
        .clone()
        .map(|value| sanitize_unit_name(value.as_str()))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| default_unit_name(request.repo_root.as_path()));

    let systemd_dir = user_systemd_dir()?;
    let service_file = systemd_dir.join(format!("{unit_name}.service"));
    let timer_file = systemd_dir.join(format!("{unit_name}.timer"));

    let current_exe = env::current_exe().map_err(|err| io_err("current_exe", err))?;
    let exec_args = scheduled_mark_args(
        request.local_only,
        request.explain,
        request.metadata_suffix.as_deref(),
    );

    let service_content = render_service_unit(
        unit_name.as_str(),
        request.repo_root.as_path(),
        current_exe.as_path(),
        exec_args.as_slice(),
    );
    let timer_content = render_timer_unit(unit_name.as_str(), request.interval_minutes);

    Ok(ServicePlan {
        unit_name,
        service_file,
        timer_file,
        service_content,
        timer_content,
    })
}

pub fn install_user_service(request: &ServiceInstallRequest) -> TideResult<ServicePlan> {
    ensure_linux("service install")?;

    let plan = plan_service(request)?;
    let unit_dir = plan
        .service_file
        .parent()
        .ok_or_else(|| TideError::Internal {
            message: format!("invalid service path: {}", plan.service_file.display()),
        })?
        .to_path_buf();

    fs::create_dir_all(&unit_dir).map_err(|err| io_err(&unit_dir, err))?;
    fs::write(&plan.service_file, plan.service_content.as_bytes())
        .map_err(|err| io_err(&plan.service_file, err))?;
    fs::write(&plan.timer_file, plan.timer_content.as_bytes())
        .map_err(|err| io_err(&plan.timer_file, err))?;

    run_systemctl_checked(&["--user", "daemon-reload"])?;
    run_systemctl_checked(&[
        "--user",
        "enable",
        "--now",
        &format!("{}.timer", plan.unit_name),
    ])?;

    Ok(plan)
}

pub fn uninstall_user_service(request: &ServiceUninstallRequest) -> TideResult<ServicePlan> {
    ensure_linux("service uninstall")?;

    let seed_request = ServiceInstallRequest {
        repo_root: request.repo_root.clone(),
        interval_minutes: 60,
        unit_name: request.unit_name.clone(),
        local_only: true,
        explain: true,
        metadata_suffix: None,
    };
    let plan = plan_service(&seed_request)?;

    let _ = run_systemctl_best_effort(&[
        "--user",
        "disable",
        "--now",
        &format!("{}.timer", plan.unit_name),
    ]);

    if plan.service_file.exists() {
        fs::remove_file(&plan.service_file).map_err(|err| io_err(&plan.service_file, err))?;
    }
    if plan.timer_file.exists() {
        fs::remove_file(&plan.timer_file).map_err(|err| io_err(&plan.timer_file, err))?;
    }

    run_systemctl_checked(&["--user", "daemon-reload"])?;

    Ok(plan)
}

pub fn default_unit_name(repo_root: &Path) -> String {
    let repo_name = repo_root
        .file_name()
        .map(|v| sanitize_unit_name(v.to_string_lossy().as_ref()))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "repo".to_string());

    let mut hash = sha2::Sha256::new();
    use sha2::Digest;
    hash.update(repo_root.to_string_lossy().as_bytes());
    let digest = hex::encode(hash.finalize());
    let short = &digest[..12];

    format!("tidemark-{repo_name}-{short}")
}

fn scheduled_mark_args(
    local_only: bool,
    explain: bool,
    metadata_suffix: Option<&str>,
) -> Vec<String> {
    let mut args = vec!["mark".to_string()];
    if explain {
        args.push("--explain".to_string());
    }
    if local_only {
        args.push("--local-only".to_string());
    }
    if let Some(tag) = metadata_suffix.map(str::trim).filter(|v| !v.is_empty()) {
        args.push("--tag".to_string());
        args.push(tag.to_string());
    }
    args
}

fn render_service_unit(
    unit_name: &str,
    repo_root: &Path,
    binary: &Path,
    exec_args: &[String],
) -> String {
    let mut exec_parts = Vec::with_capacity(exec_args.len() + 1);
    exec_parts.push(systemd_quote(binary.to_string_lossy().as_ref()));
    for arg in exec_args {
        exec_parts.push(systemd_quote(arg));
    }
    let exec_start = exec_parts.join(" ");

    format!(
        "[Unit]\nDescription=TideMark scheduled resolver ({unit_name})\nAfter=network-online.target\n\n[Service]\nType=oneshot\nWorkingDirectory={}\nExecStart={}\nStandardOutput=journal\nStandardError=journal\n\n",
        systemd_quote(repo_root.to_string_lossy().as_ref()),
        exec_start,
    )
}

fn render_timer_unit(unit_name: &str, interval_minutes: u32) -> String {
    format!(
        "[Unit]\nDescription=TideMark schedule ({unit_name})\n\n[Timer]\nOnBootSec=2min\nOnUnitActiveSec={}min\nAccuracySec=1s\nPersistent=true\nUnit={}.service\n\n[Install]\nWantedBy=timers.target\n",
        interval_minutes, unit_name
    )
}

fn sanitize_unit_name(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

fn systemd_quote(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn user_systemd_dir() -> TideResult<PathBuf> {
    let home = env::var_os("HOME").ok_or(TideError::MissingHomeDirectory)?;
    Ok(PathBuf::from(home).join(".config/systemd/user"))
}

fn ensure_linux(feature: &str) -> TideResult<()> {
    if cfg!(target_os = "linux") {
        Ok(())
    } else {
        Err(TideError::UnsupportedPlatform {
            feature: feature.to_string(),
        })
    }
}

fn validate_interval(minutes: u32) -> TideResult<()> {
    if minutes == 0 {
        return Err(TideError::InvalidServiceInterval { minutes });
    }
    Ok(())
}

fn run_systemctl_checked(args: &[&str]) -> TideResult<()> {
    let output = Command::new("systemctl")
        .args(args)
        .output()
        .map_err(|err| io_err("systemctl", err))?;

    if output.status.success() {
        return Ok(());
    }

    Err(TideError::SystemCommand {
        program: "systemctl".to_string(),
        args: args.iter().map(|v| v.to_string()).collect(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        code: output.status.code(),
    })
}

fn run_systemctl_best_effort(args: &[&str]) -> TideResult<()> {
    let output = Command::new("systemctl")
        .args(args)
        .output()
        .map_err(|err| io_err("systemctl", err))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
    if stderr.contains("not loaded") || stderr.contains("not found") {
        return Ok(());
    }

    Err(TideError::SystemCommand {
        program: "systemctl".to_string(),
        args: args.iter().map(|v| v.to_string()).collect(),
        stderr: stderr.trim().to_string(),
        code: output.status.code(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_name_is_stable_for_same_path() {
        let path = Path::new("/tmp/example-repo");
        let a = default_unit_name(path);
        let b = default_unit_name(path);
        assert_eq!(a, b);
        assert!(a.starts_with("tidemark-example-repo-"));
    }

    #[test]
    fn timer_plan_contains_interval() {
        let req = ServiceInstallRequest {
            repo_root: PathBuf::from("/tmp/repo"),
            interval_minutes: 15,
            unit_name: Some("custom_name".to_string()),
            local_only: true,
            explain: true,
            metadata_suffix: Some("dev".to_string()),
        };

        let plan = plan_service(&req).expect("plan should succeed");
        assert_eq!(plan.unit_name, "custom_name");
        assert!(plan.timer_content.contains("OnUnitActiveSec=15min"));
        assert!(plan.service_content.contains("--local-only"));
        assert!(plan.service_content.contains("--explain"));
        assert!(plan.service_content.contains("--tag"));
    }

    #[test]
    fn invalid_interval_rejected() {
        let req = ServiceInstallRequest {
            repo_root: PathBuf::from("/tmp/repo"),
            interval_minutes: 0,
            unit_name: None,
            local_only: true,
            explain: true,
            metadata_suffix: None,
        };

        let err = plan_service(&req).unwrap_err();
        match err {
            TideError::InvalidServiceInterval { minutes } => assert_eq!(minutes, 0),
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn sanitization_keeps_safe_charset() {
        assert_eq!(sanitize_unit_name("Tide Mark@Repo"), "tide-mark-repo");
    }
}
