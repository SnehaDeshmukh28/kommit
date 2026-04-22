mod diff;
mod hook;
mod interactive;
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
                hook::HookStatus::KommitNotInPath => {
                    eprintln!("Error: kommit is not in your PATH.");
                    eprintln!("Add the binary to your PATH first, then run `kommit init` again.");
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
            println!("  Avg length     : {} chars\n", profile.avg_length);

            match diff::get_staged_diff() {
                Ok(d) if d.is_empty => {
                    println!("Nothing staged. Run `git add` first.");
                }
                Ok(d) => {
                    println!("Generating commit message...\n");

                    let start = std::time::Instant::now();

                    let req = model::GenerateRequest {
                        diff: d.raw.clone(),
                        files_changed: d.files_changed.clone(),
                        style_hint: Some(style_hint.clone()),
                    };

                    let mut response = model::generate_stub(&req);
                    let elapsed = start.elapsed();
                    println!("  Generated in {:.1}s\n", elapsed.as_secs_f32());

                    loop {
                        match interactive::prompt_user(&response.message) {
                            interactive::UserChoice::Accept => {
                                match interactive::commit_with_message(&response.message) {
                                    Ok(_) => break,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        break;
                                    }
                                }
                            }
                            interactive::UserChoice::Edit(new_msg) => {
                                match interactive::commit_with_message(&new_msg) {
                                    Ok(_) => break,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        break;
                                    }
                                }
                            }
                            interactive::UserChoice::Reject => {
                                println!("Cancelled. Nothing committed.");
                                break;
                            }
                            interactive::UserChoice::Regenerate => {
                                println!("Regenerating...\n");
                                let regen_start = std::time::Instant::now();
                                let new_req = model::GenerateRequest {
                                    diff: d.raw.clone(),
                                    files_changed: d.files_changed.clone(),
                                    style_hint: Some(style_hint.clone()),
                                };
                                response = model::generate_stub(&new_req);
                                let regen_elapsed = regen_start.elapsed();
                                println!("  Generated in {:.1}s\n", regen_elapsed.as_secs_f32());
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading diff: {}", e);
                }
            }
        }
    }
}
