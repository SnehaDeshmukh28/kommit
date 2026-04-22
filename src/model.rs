use serde::{Deserialize, Serialize};

pub struct GenerateRequest {
    pub diff: String,
    pub files_changed: Vec<String>,
    pub style_hint: Option<String>,
}

pub struct GenerateResponse {
    pub message: String,
    #[allow(dead_code)]
    pub confidence: f32,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub fn build_prompt(req: &GenerateRequest) -> String {
    let config = crate::config::load();

    let files_summary = if req.files_changed.is_empty() {
        "unknown files".to_string()
    } else {
        req.files_changed.join(", ")
    };

    let diff_preview = if req.diff.len() > config.max_diff_chars {
        format!("{}... [truncated]", &req.diff[..config.max_diff_chars])
    } else {
        req.diff.clone()
    };

    let style = req
        .style_hint
        .as_deref()
        .unwrap_or("conventional commits format like: feat(scope): description");

    format!(
        r#"You are a git commit message generator.
Generate ONE commit message for the following staged changes.
Use this style: {style}
Rules:
- Start with a type: feat, fix, chore, docs, refactor, test
- Keep it under 72 characters
- Be specific, not generic
- No period at the end
- Output ONLY the commit message, nothing else

Changed files: {files_summary}

Diff:
{diff_preview}

Commit message:"#
    )
}

pub fn generate(req: &GenerateRequest) -> GenerateResponse {
    let config = crate::config::load();
    let prompt = build_prompt(req);

    match call_ollama(&prompt, &config.ollama_url, &config.model) {
        Ok(message) => GenerateResponse {
            message: message.trim().to_string(),
            confidence: 1.0,
        },
        Err(e) => {
            eprintln!("Model error: {}", e);
            eprintln!("Make sure ollama is running: ollama serve");
            eprintln!("Config file: {}", crate::config::show_path());
            GenerateResponse {
                message: "chore: update implementation".to_string(),
                confidence: 0.0,
            }
        }
    }
}

fn call_ollama(prompt: &str, base_url: &str, model: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    let body = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let url = format!("{}/api/generate", base_url);

    let response = client
        .post(&url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .map_err(|e| format!("Failed to reach ollama at {}: {}", base_url, e))?;

    let ollama_response: OllamaResponse = response
        .json()
        .map_err(|e| format!("Failed to parse ollama response: {}", e))?;

    Ok(ollama_response.response)
}

pub fn generate_stub(req: &GenerateRequest) -> GenerateResponse {
    generate(req)
}
