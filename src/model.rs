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

Changed files: {files_summary}

Diff:
{diff_preview}

Commit message:"#
    )
}

pub fn generate_stub(req: &GenerateRequest) -> GenerateResponse {
    let prompt = build_prompt(req);

    println!("[debug] Prompt built ({} chars)", prompt.len());
    println!("[debug] Model inference will go here");

    let file_hint = req
        .files_changed
        .first()
        .map(|f| {
            if f.contains("test") {
                "test"
            } else if f.contains("readme") || f.contains("README") {
                "docs"
            } else {
                "feat"
            }
        })
        .unwrap_or("chore");

    let scope = req
        .files_changed
        .first()
        .and_then(|f| f.split('/').nth(1))
        .unwrap_or("core");

    GenerateResponse {
        message: format!("{}({}): update implementation", file_hint, scope),
        confidence: 0.0,
    }
}
