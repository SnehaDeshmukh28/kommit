use std::process::Command;

pub struct Diff {
    pub raw: String,
    pub files_changed: Vec<String>,
    pub is_empty: bool,
}

pub fn get_staged_diff() -> Result<Diff, String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--stat"])
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    let stat = String::from_utf8_lossy(&output.stdout).to_string();

    let full_output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    let raw = String::from_utf8_lossy(&full_output.stdout).to_string();

    let files_changed: Vec<String> = stat
        .lines()
        .filter(|line| line.contains('|'))
        .map(|line| line.split('|').next().unwrap_or("").trim().to_string())
        .collect();

    let is_empty = raw.trim().is_empty();

    Ok(Diff {
        raw,
        files_changed,
        is_empty,
    })
}