use serde::{Deserialize, Serialize};

pub struct GenerateRequest {
    pub diff: String,
    pub files_changed: Vec<String>,
    pub style_hint: Option<String>,
    pub include_body: bool,
}

pub struct GenerateResponse {
    pub message: String,
    pub body: Option<String>,
    #[allow(dead_code)]
    pub confidence: f32,
}

impl GenerateResponse {
    pub fn full_message(&self) -> String {
        match &self.body {
            Some(b) if !b.is_empty() => format!("{}\n\n{}", self.message, b),
            _ => self.message.clone(),
        }
    }
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

    if req.include_body {
        format!(
            r#"You are a git commit message generator.
Generate a commit message for the following staged changes.
Use this style: {style}

You MUST use exactly this format with a blank line between subject and body:

feat(scope): short description under 72 chars

This explains WHY the change was made in 2-3 sentences. Focus on
the reasoning and motivation, not just what changed.

Rules:
- Subject line: under 72 chars, no period at end
- Blank line between subject and body (mandatory)
- Body: explain WHY, not what
- Output ONLY the commit message, nothing else

Changed files: {files_summary}

Diff:
{diff_preview}

Commit message (subject, blank line, then body):"#
        )
    } else {
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
}

pub fn generate(req: &GenerateRequest) -> GenerateResponse {
    let prompt = build_prompt(req);

    match call_ollama(&prompt) {
        Ok(raw) => parse_response(raw),
        Err(e) => {
            eprintln!("Model error: {}", e);
            eprintln!("Make sure ollama is running: ollama serve");
            eprintln!("Config file: {}", crate::config::show_path());
            GenerateResponse {
                message: "chore: update implementation".to_string(),
                body: None,
                confidence: 0.0,
            }
        }
    }
}

fn parse_response(raw: String) -> GenerateResponse {
    let trimmed = raw.trim().to_string();

    let cleaned = trimmed
        .trim_end_matches("getBody")
        .trim_end_matches("getMessage")
        .trim_end_matches("getSubject")
        .trim()
        .to_string();

    let mut parts = cleaned.splitn(3, '\n');

    let subject = parts.next().unwrap_or("").trim().to_string();
    let second = parts.next().unwrap_or("").trim().to_string();
    let rest = parts.next().unwrap_or("").trim().to_string();

    let body = if second.is_empty() && !rest.is_empty() {
        Some(rest)
    } else if !second.is_empty() {
        let full_body = if rest.is_empty() {
            second
        } else {
            format!("{}\n{}", second, rest)
        };
        Some(full_body)
    } else {
        None
    };

    GenerateResponse {
        message: subject,
        body,
        confidence: 1.0,
    }
}

fn call_ollama(prompt: &str) -> Result<String, String> {
    let config = crate::config::load();
    let client = reqwest::blocking::Client::new();

    let body = OllamaRequest {
        model: config.model.clone(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let url = format!("{}/api/generate", config.ollama_url);

    let response = client
        .post(&url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .map_err(|e| format!("Failed to reach ollama at {}: {}", config.ollama_url, e))?;

    let ollama_response: OllamaResponse = response
        .json()
        .map_err(|e| format!("Failed to parse ollama response: {}", e))?;

    Ok(ollama_response.response)
}

pub fn generate_stub(req: &GenerateRequest) -> GenerateResponse {
    generate(req)
}
