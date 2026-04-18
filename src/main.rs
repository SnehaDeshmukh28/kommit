mod diff;
mod model;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kommit", version, about = "AI-powered git commit messages, fully local and offline")]
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
            println!("kommit: setting up in this repo...");
        }
        Commands::Generate => {
            match diff::get_staged_diff() {
                Ok(d) if d.is_empty => {
                    println!("Nothing staged. Run `git add` first.");
                }
                Ok(d) => {
                    let req = model::GenerateRequest {
                        diff: d.raw,
                        files_changed: d.files_changed,
                        style_hint: None,
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