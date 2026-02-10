// axum-app-create: A CLI tool to scaffold Axum web applications
//
// This tool generates new Axum projects with sensible defaults and optional features.

use axum_app_create::cli::{is_non_interactive, prompts::prompt_project_config};
use axum_app_create::config::DatabaseOption;
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
    /// Project name (positional argument or --project-name)
    #[arg(value_name = "PROJECT_NAME")]
    project_name: Option<String>,

    /// Author name for generated project
    #[arg(long)]
    author: Option<String>,

    /// Database support: none, postgresql, sqlite, or both
    #[arg(long, value_name = "TYPE")]
    database: Option<String>,

    /// Enable JWT authentication
    #[arg(long)]
    auth: bool,

    /// Enable biz-error integration
    #[arg(long)]
    biz_error: bool,

    /// Default log level: trace, debug, info, warn, error
    #[arg(long, value_name = "LEVEL")]
    log_level: Option<String>,

    /// Force overwrite if target directory exists
    #[arg(long)]
    force: bool,

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

    // Parse database option from CLI flag
    let cli_database = args.database.as_deref().map(|d| match d {
        "postgresql" | "postgres" | "pg" => DatabaseOption::PostgreSQL,
        "sqlite" => DatabaseOption::SQLite,
        "both" => DatabaseOption::Both,
        "none" => DatabaseOption::None,
        other => {
            eprintln!(
                "\n❌ Invalid database option: '{}'\n\
                 💡 Valid options: none, postgresql, sqlite, both",
                other
            );
            std::process::exit(1);
        }
    });

    // Validate log level if provided
    if let Some(ref level) = args.log_level
        && !["trace", "debug", "info", "warn", "error"].contains(&level.as_str())
    {
        eprintln!(
            "\n❌ Invalid log level: '{}'\n\
                 💡 Valid levels: trace, debug, info, warn, error",
            level
        );
        std::process::exit(1);
    }

    // Determine if we're in interactive mode
    let interactive = !is_non_interactive(args.non_interactive);

    // Build CLI overrides
    let cli_overrides = axum_app_create::cli::prompts::CliOverrides {
        database: cli_database,
        auth: if args.auth { Some(true) } else { None },
        biz_error: if args.biz_error { Some(true) } else { None },
        log_level: args.log_level,
        author: args.author,
    };

    // Get project configuration
    let config =
        match prompt_project_config(interactive, args.project_name, Some(cli_overrides)) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("\n❌ {}", e);
                std::process::exit(1);
            }
        };

    // Determine project directory
    let project_dir = PathBuf::from(&config.project_name);

    // Generate project
    match generate_project(&project_dir, &config, interactive, args.force) {
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
