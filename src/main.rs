// create-axum-app: A CLI tool to scaffold Axum web applications
//
// This tool generates new Axum projects with sensible defaults and optional features.

use clap::Parser;
use tracing::info;

/// Simple CLI tool to scaffold Axum web applications
#[derive(Parser, Debug)]
#[command(name = "create-axum-app")]
#[command(about = "Scaffold a new Axum web application", long_about = None)]
#[command(version = "0.1.0")]
struct CliArgs {
    /// Project name
    #[arg(short, long)]
    project_name: Option<String>,

    /// Enable database support (none, postgresql, sqlite, both)
    #[arg(long, value_name = "DATABASE")]
    database: Option<String>,

    /// Enable JWT authentication
    #[arg(long)]
    auth: bool,

    /// Enable business error handling
    #[arg(long)]
    biz_error: bool,

    /// Logging level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL")]
    log_level: Option<String>,

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

    info!("create-axum-app CLI Tool");
    info!("Implementation in progress...");

    println!("\n🚀 create-axum-app CLI Tool");
    println!("Version: 0.1.0");
    println!("\nThis tool will scaffold a new Axum web application.");
    println!("\nProject configuration:");
    if let Some(name) = &args.project_name {
        println!("  Project name: {}", name);
    } else {
        println!("  Project name: <will prompt>");
    }
    println!("  Database: {:?}", args.database);
    println!("  Authentication: {}", args.auth);
    println!("  Business errors: {}", args.biz_error);
    println!("  Non-interactive: {}", args.non_interactive);

    println!("\n⚠️  Implementation in progress...");
    println!("Full functionality coming soon!");

    Ok(())
}
