// axum-app-create: A CLI tool to scaffold Axum web applications
//
// This tool generates new Axum projects with sensible defaults and optional features.

use axum_app_create::cli::{is_non_interactive, prompts::prompt_project_config};
use axum_app_create::error::CliError;
use axum_app_create::generator::project::{generate_project, get_success_message};
use axum_app_create::utils::rust_toolchain::check_rust_toolchain;
use clap::Parser;
use std::path::PathBuf;

/// Simple CLI tool to scaffold Axum web applications
#[derive(Parser, Debug)]
#[command(name = "axum-app-create")]
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

/// Format error message with troubleshooting guidance
fn format_error_message(error: &CliError) -> String {
    match error {
        CliError::Io(_) | CliError::Git(_) | CliError::Template(_) | CliError::Generation(_) => {
            format!(
                "{}\n\n\
                 🔍 故障排查 / Troubleshooting:\n\
                 1. 检查文件系统权限 / Check file system permissions\n\
                 2. 确保磁盘空间充足 / Ensure sufficient disk space\n\
                 3. 查看日志获取更多信息 / Check logs for more details: RUST_LOG=debug\n\
                 4. 查看帮助 / View help: axum-app-create --help\n\
                 5. 提交bug报告 / Report bug: https://github.com/yourusername/axum-app-create/issues",
                error
            )
        }
        _ => error.to_string(),
    }
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

    println!("\n🦀 axum-app-create CLI Tool v0.1.0");

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
            eprintln!("\n❌ {}", e);
            std::process::exit(1);
        }
    };

    // Determine project directory
    let project_dir = PathBuf::from(&config.project_name);

    // Generate project
    match generate_project(&project_dir, &config, interactive) {
        Ok(()) => {
            // Print success message
            let message = get_success_message(&project_dir, &config.project_name);
            println!("{}", message);
        }
        Err(e) => {
            eprintln!("\n❌ {}", format_error_message(&e));
            std::process::exit(1);
        }
    }

    Ok(())
}
