use std::process::Command;

pub struct StyleProfile {
    pub preferred_type: String,
    pub avg_length: usize,
    pub uses_scope: bool,
    #[allow(dead_code)]
    pub sample_messages: Vec<String>,
}

impl Default for StyleProfile {
    fn default() -> Self {
        StyleProfile {
            preferred_type: "feat".to_string(),
            avg_length: 50,
            uses_scope: false,
            sample_messages: vec![],
        }
    }
}

pub fn learn_from_git_log() -> StyleProfile {
    let output = Command::new("git")
        .args(["log", "--oneline", "-50", "--pretty=format:%s"])
        .output();

    let Ok(output) = output else {
        return StyleProfile::default();
    };

    let log = String::from_utf8_lossy(&output.stdout);
    let messages: Vec<&str> = log.lines().collect();

    if messages.is_empty() {
        return StyleProfile::default();
    }

    let uses_scope = messages.iter().any(|m| m.contains('(') && m.contains(')'));

    let avg_length = messages
        .iter()
        .map(|m| m.len())
        .sum::<usize>()
        .checked_div(messages.len())
        .unwrap_or(50);

    let preferred_type = ["feat", "fix", "chore", "refactor", "docs", "test"]
        .iter()
        .max_by_key(|t| messages.iter().filter(|m| m.starts_with(*t)).count())
        .unwrap_or(&"feat")
        .to_string();

    let sample_messages = messages.iter().take(5).map(|s| s.to_string()).collect();

    StyleProfile {
        preferred_type,
        avg_length,
        uses_scope,
        sample_messages,
    }
}

pub fn build_style_hint(profile: &StyleProfile) -> String {
    let scope_note = if profile.uses_scope {
        "include a scope in parentheses like feat(scope):"
    } else {
        "no scope needed, just type: description"
    };

    format!(
        "conventional commits, prefer '{}' type, keep under {} chars, {}",
        profile.preferred_type, profile.avg_length, scope_note
    )
}
