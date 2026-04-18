mod diff;

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
                    println!("Found {} changed file(s):", d.files_changed.len());
                    for file in &d.files_changed {
                        println!("  - {}", file);
                    }
                    println!("\nDiff preview ({} chars)", d.raw.len());
                }
                Err(e) => {
                    eprintln!("Error reading diff: {}", e);
                }
            }
        }
    }
}