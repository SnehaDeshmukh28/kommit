use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub enum HookStatus {
    Installed,
    AlreadyInstalled,
    NotAGitRepo,
    Failed(String),
}

fn find_git_root() -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Some(PathBuf::from(path))
}

pub fn install_hook() -> HookStatus {
    let Some(git_root) = find_git_root() else {
        return HookStatus::NotAGitRepo;
    };

    let hook_path = git_root
        .join(".git")
        .join("hooks")
        .join("prepare-commit-msg");

    if hook_path.exists() {
        let existing = fs::read_to_string(&hook_path).unwrap_or_default();
        if existing.contains("kommit") {
            return HookStatus::AlreadyInstalled;
        }
    }

    let hook_script = "#!/bin/sh\nkommit generate --hook \"$1\"\n";

    match fs::write(&hook_path, hook_script) {
        Ok(_) => {
            set_executable(&hook_path);
            HookStatus::Installed
        }
        Err(e) => HookStatus::Failed(e.to_string()),
    }
}

#[cfg(unix)]
fn set_executable(path: &PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(mut perms) = fs::metadata(path).map(|m| m.permissions()) {
        perms.set_mode(0o755);
        let _ = fs::set_permissions(path, perms);
    }
}

#[cfg(windows)]
fn set_executable(_path: &PathBuf) {
    // Windows doesn't use Unix permissions
    // Git for Windows handles executable bits separately
}
