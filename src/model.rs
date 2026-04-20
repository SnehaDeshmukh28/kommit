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
    let files_summary = if req.files_changed.is_empty() {
        "unknown files".to_string()
    } else {
        req.files_changed.join(", ")
    };

    let diff_preview = if req.diff.len() > 3000 {
        format!("{}... [truncated]", &req.diff[..3000])
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
    let prompt = build_prompt(req);

    match call_ollama(&prompt) {
        Ok(message) => GenerateResponse {
            message: message.trim().to_string(),
            confidence: 1.0,
        },
        Err(e) => {
            eprintln!("Model error: {}", e);
            eprintln!("Make sure ollama is running: ollama serve");
            GenerateResponse {
                message: "chore: update implementation".to_string(),
                confidence: 0.0,
            }
        }
    }
}

fn call_ollama(prompt: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    let body = OllamaRequest {
        model: "qwen2.5-coder:1.5b".to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .map_err(|e| format!("Failed to reach ollama: {}", e))?;

    let ollama_response: OllamaResponse = response
        .json()
        .map_err(|e| format!("Failed to parse ollama response: {}", e))?;

    Ok(ollama_response.response)
}

pub fn generate_stub(req: &GenerateRequest) -> GenerateResponse {
    generate(req)
}
