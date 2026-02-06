// create-axum-app: A CLI tool to scaffold Axum web applications
//
// This tool generates new Axum projects with sensible defaults and optional features.

use clap::Parser;
use create_axum_app::cli::{is_non_interactive, prompts::prompt_project_config};
use create_axum_app::generator::project::{generate_project, get_success_message};
use create_axum_app::utils::rust_toolchain::check_rust_toolchain;
use std::path::PathBuf;

/// Simple CLI tool to scaffold Axum web applications
#[derive(Parser, Debug)]
#[command(name = "create-axum-app")]
#[command(about = "Scaffold a new Axum web application", long_about = None)]
#[command(version = "0.1.0")]
struct CliArgs {
    /// Project name
    #[arg(short, long)]
    project_name: Option<String>,

    /// Author name for generated project
    #[arg(long)]
    author: Option<String>,

    /// Non-interactive mode (fail if required values missing)
    #[arg(long)]
    non_interactive: bool,
}

fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = CliArgs::parse();

    println!("\n🦀 create-axum-app CLI Tool v0.1.0");

    // Check Rust toolchain
    if let Err(e) = check_rust_toolchain() {
        eprintln!("\n❌ {}", e);
        std::process::exit(1);
    }

    // Determine if we're in interactive mode
    let interactive = !is_non_interactive(args.non_interactive);

    // Get project configuration
    let config = match prompt_project_config(interactive, args.project_name) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("\n❌ Error: {}", e);
            std::process::exit(1);
        }
    };

    // Determine project directory
    let project_dir = PathBuf::from(&config.project_name);

    // Generate project
    match generate_project(&project_dir, &config) {
        Ok(()) => {
            // Print success message
            let message = get_success_message(&project_dir, &config.project_name);
            println!("{}", message);
        }
        Err(e) => {
            eprintln!("\n❌ Failed to generate project: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
