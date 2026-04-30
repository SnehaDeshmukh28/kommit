mod config;
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
    #[command(arg_required_else_help = false)]
    Generate {
        /// Include reasoning body in commit message
        #[arg(long, short)]
        body: bool,
    },
    /// Show current config and config file location
    Config,
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
        Commands::Generate { body } => {
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
                    if !d.skipped_files.is_empty() {
                        println!(
                            "  Skipped {} junk file(s): {}\n",
                            d.skipped_files.len(),
                            d.skipped_files.join(", ")
                        );
                    }

                    if body {
                        println!("Generating commit message with reasoning...\n");
                    } else {
                        println!("Generating commit message...\n");
                    }

                    let start = std::time::Instant::now();

                    let req = model::GenerateRequest {
                        diff: d.raw.clone(),
                        files_changed: d.files_changed.clone(),
                        style_hint: Some(style_hint.clone()),
                        include_body: body,
                    };

                    let mut response = model::generate_stub(&req);
                    let elapsed = start.elapsed();
                    println!("  Generated in {:.1}s\n", elapsed.as_secs_f32());

                    loop {
                        match interactive::prompt_user(&response.full_message()) {
                            interactive::UserChoice::Accept => {
                                match interactive::commit_with_message(&response.full_message()) {
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
                                    include_body: body,
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
        Commands::Config => {
            let config = config::load();
            println!("Config file: {}", config::show_path());
            println!();
            println!("  model          = {}", config.model);
            println!("  ollama_url     = {}", config.ollama_url);
            println!("  max_diff_chars = {}", config.max_diff_chars);
            println!();
            println!("To customize, create the config file and add:");
            println!("  model = \"llama3.2:3b\"");
            println!("  ollama_url = \"http://localhost:11434\"");
            println!("  max_diff_chars = 4000");
        }
    }
}
