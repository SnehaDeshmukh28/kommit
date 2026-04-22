use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub ollama_url: String,
    pub model: String,
    pub max_diff_chars: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ollama_url: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:1.5b".to_string(),
            max_diff_chars: 3000,
        }
    }
}

fn config_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(home).join(".kommit").join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Config::default(),
    };

    let mut config = Config::default();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match key {
                "model" => config.model = value.to_string(),
                "ollama_url" => config.ollama_url = value.to_string(),
                "max_diff_chars" => {
                    if let Ok(n) = value.parse() {
                        config.max_diff_chars = n;
                    }
                }
                _ => {}
            }
        }
    }

    config
}

pub fn show_path() -> String {
    config_path().display().to_string()
}
