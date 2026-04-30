use std::io::{self, BufRead, Write};

pub enum UserChoice {
    Accept,
    Edit(String),
    Reject,
    Regenerate,
}

pub fn prompt_user(suggested: &str) -> UserChoice {
    loop {
        println!();

        let parts: Vec<&str> = suggested.splitn(2, "\n\n").collect();
        let subject = parts[0].trim();

        println!("  Suggested message:");
        println!();
        println!("    {}", subject);

        if parts.len() > 1 {
            let body = parts[1].trim();
            if !body.is_empty() {
                println!();
                println!("  Reasoning:");
                println!();
                for line in body.lines() {
                    println!("    {}", line);
                }
            }
        }

        println!();
        println!("  y  accept    e  edit    r  regenerate    n  cancel");
        println!();
        print!("  Your choice: ");

        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let line = stdin.lock().lines().next();

        let input = match line {
            Some(Ok(l)) => l.trim().to_lowercase(),
            _ => continue,
        };

        match input.as_str() {
            "y" | "yes" | "" => return UserChoice::Accept,
            "n" | "no" => return UserChoice::Reject,
            "r" => return UserChoice::Regenerate,
            "e" | "edit" => {
                print!("  New message: ");
                io::stdout().flush().unwrap();

                let edited = stdin
                    .lock()
                    .lines()
                    .next()
                    .and_then(|l| l.ok())
                    .unwrap_or_default();

                let trimmed = edited.trim().to_string();
                if trimmed.is_empty() {
                    println!("  Empty message, try again.");
                    continue;
                }
                return UserChoice::Edit(trimmed);
            }
            _ => {
                println!("  Please enter y, e, r, or n.");
                continue;
            }
        }
    }
}

pub fn commit_with_message(message: &str) -> Result<(), String> {
    let output = std::process::Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .map_err(|e| format!("Failed to run git commit: {}", e))?;

    if output.status.success() {
        println!("\nCommitted: {}", message);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("git commit failed: {}", stderr))
    }
}
