use std::process::Command;

pub struct Diff {
    pub raw: String,
    pub files_changed: Vec<String>,
    pub is_empty: bool,
    pub skipped_files: Vec<String>,
}

const JUNK_EXTENSIONS: &[&str] = &[
    ".lock", ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".woff", ".woff2", ".ttf", ".eot",
    ".pdf", ".zip", ".tar", ".gz", ".exe", ".dll", ".so", ".dylib",
];

const JUNK_FILENAMES: &[&str] = &[
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "Cargo.lock",
    "Gemfile.lock",
    "poetry.lock",
    "composer.lock",
    "packages.lock.json",
    "pubspec.lock",
];

fn is_junk_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    let filename = lower.split('/').last().unwrap_or(&lower);

    if JUNK_FILENAMES.iter().any(|j| filename == *j) {
        return true;
    }

    if JUNK_EXTENSIONS.iter().any(|ext| lower.ends_with(ext)) {
        return true;
    }

    false
}

pub fn get_staged_diff() -> Result<Diff, String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--stat"])
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    let stat = String::from_utf8_lossy(&output.stdout).to_string();

    let all_files: Vec<String> = stat
        .lines()
        .filter(|line| line.contains('|'))
        .map(|line| line.split('|').next().unwrap_or("").trim().to_string())
        .collect();

    let (good_files, skipped_files): (Vec<String>, Vec<String>) =
        all_files.into_iter().partition(|f| !is_junk_file(f));

    if good_files.is_empty() && !skipped_files.is_empty() {
        return Ok(Diff {
            raw: String::new(),
            files_changed: vec![],
            is_empty: true,
            skipped_files,
        });
    }

    let mut diff_args = vec!["diff", "--cached", "--"];
    let good_file_refs: Vec<&str> = good_files.iter().map(|s| s.as_str()).collect();
    diff_args.extend_from_slice(&good_file_refs);

    let full_output = Command::new("git")
        .args(&diff_args)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    let raw = String::from_utf8_lossy(&full_output.stdout).to_string();
    let is_empty = raw.trim().is_empty();

    Ok(Diff {
        raw,
        files_changed: good_files,
        is_empty,
        skipped_files,
    })
}
