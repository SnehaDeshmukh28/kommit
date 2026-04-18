use clap::{Parser, Subcommand};

/// kommit — AI-powered git commit messages, fully local
#[derive(Parser)]
#[command(name = "kommit", version, about, long_about = None)]
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
            println!("kommit: generating commit message...");
        }
    }
}