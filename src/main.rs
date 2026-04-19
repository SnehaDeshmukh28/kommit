mod diff;
mod hook;
mod model;
mod style;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kommit",
    version,
    about = "AI-powered git commit messages, fully local and offline"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up kommit in the current git repo
    Init,
    /// Generate a commit message from staged changes
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Setting up kommit...");
            match hook::install_hook() {
                hook::HookStatus::Installed => {
                    println!("Hook installed. kommit will now suggest messages on every commit.");
                }
                hook::HookStatus::AlreadyInstalled => {
                    println!("Hook already installed. You're good to go.");
                }
                hook::HookStatus::NotAGitRepo => {
                    eprintln!("Error: not inside a git repository.");
                    std::process::exit(1);
                }
                hook::HookStatus::Failed(e) => {
                    eprintln!("Failed to install hook: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Generate => {
            let profile = style::learn_from_git_log();
            let style_hint = style::build_style_hint(&profile);

            println!("Learning from your last commits...");
            println!("  Preferred type : {}", profile.preferred_type);
            println!("  Uses scope     : {}", profile.uses_scope);
            println!("  Avg length     : {} chars", profile.avg_length);

            match diff::get_staged_diff() {
                Ok(d) if d.is_empty => {
                    println!("Nothing staged. Run `git add` first.");
                }
                Ok(d) => {
                    let req = model::GenerateRequest {
                        diff: d.raw,
                        files_changed: d.files_changed,
                        style_hint: Some(style_hint),
                    };
                    let response = model::generate_stub(&req);
                    println!("\nSuggested commit message:");
                    println!("\n  {}\n", response.message);
                    println!("Accept? [y/n/e to edit]: ");
                }
                Err(e) => {
                    eprintln!("Error reading diff: {}", e);
                }
            }
        }
    }
}
